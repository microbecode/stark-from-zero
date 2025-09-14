use crate::evaluation_domain::EvaluationDomain;
use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::merkle_tree::MerkleTree;
use crate::polynomial::interpolate::lagrange_interpolation;
use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;
use crate::verifier::{SamplingData, StarkProof};

/// Low Degree Extension: Interpolate trace columns and evaluate at larger domain
pub fn extend_trace(
    trace: &Trace,
    field: FiniteField,
    extension_factor: usize,
) -> Vec<Vec<FiniteFieldElement>> {
    println!("🔄 Performing Low Degree Extension...");

    let original_size = trace.num_rows();
    let extended_size = original_size * extension_factor;

    println!("   Original trace size: {} steps", original_size);
    println!(
        "   Extended trace size: {} steps ({}x extension)",
        extended_size, extension_factor
    );

    // Create evaluation domain for the extended size
    let eval_domain = EvaluationDomain::new_linear(field, extended_size);

    // For each column in the trace, interpolate and extend
    let mut extended_trace = Vec::new();

    for col in 0..trace.num_columns() {
        println!("   Extending column {}...", col);

        // Get the original column values
        let original_column = trace.get_column(col);

        // Create interpolation points: (step, value) pairs
        let mut points = Vec::new();
        for (step, &value) in original_column.iter().enumerate() {
            points.push((step as i128, value));
        }

        // Interpolate to get polynomial
        let poly = lagrange_interpolation(&points);

        // Evaluate polynomial at extended domain
        let mut extended_column = Vec::new();
        for i in 0..extended_size {
            let point = eval_domain.element(i);
            let value = poly.evaluate(point);
            extended_column.push(value);
        }

        extended_trace.push(extended_column);
    }

    println!("   ✅ LDE complete!");
    extended_trace
}

/// Create constraint polynomial: C(x) = F(x) - F(x-1) - F(x-2)
/// This polynomial should evaluate to 0 at all valid computation steps
fn create_fibonacci_constraint_poly(
    trace: &Trace,
    field: FiniteField,
) -> (Polynomial, EvaluationDomain) {
    println!("🔧 Creating Fibonacci constraint polynomial...");

    let original_size = trace.num_rows();
    let eval_domain = EvaluationDomain::new_linear(field, original_size);

    // Create polynomials for each column: F(x-2), F(x-1), F(x)
    let mut column_polys = Vec::new();

    for col in 0..trace.num_columns() {
        let column_values = trace.get_column(col);
        let mut points = Vec::new();
        for (step, &value) in column_values.iter().enumerate() {
            points.push((step as i128, value));
        }
        let poly = lagrange_interpolation(&points);
        column_polys.push(poly);
    }

    // C(x) = F(x) - F(x-1) - F(x-2)
    // For this simplified version, we'll create a constraint polynomial
    // that evaluates to 0 at all points where the Fibonacci rule should hold

    // Create a polynomial that represents the constraint residuals
    let mut constraint_points = Vec::new();

    // For steps 0 and 1, the constraint is trivially satisfied (no previous terms)
    constraint_points.push((0, 0));
    if original_size > 1 {
        constraint_points.push((1, 0));
    }

    // For steps 2 and beyond, compute the actual constraint residual
    for step in 2..original_size {
        let f_n_minus_2 = trace.get(step, 0).unwrap();
        let f_n_minus_1 = trace.get(step, 1).unwrap();
        let f_n = trace.get(step, 2).unwrap();
        let residual = f_n - (f_n_minus_1 + f_n_minus_2);
        constraint_points.push((step as i128, residual));
    }

    // Interpolate the constraint residuals to get the constraint polynomial
    let constraint_poly = lagrange_interpolation(&constraint_points);

    println!(
        "   ✅ Constraint polynomial created (degree: {})",
        constraint_poly.degree()
    );

    (constraint_poly, eval_domain)
}

/// Step 2: Prover with Low Degree Extension
pub fn prove_fibonacci(trace: Trace, field: FiniteField) -> StarkProof {
    println!("🔍 Starting STARK proof generation...");
    println!(
        "   Trace size: {} rows × {} columns",
        trace.num_rows(),
        trace.num_columns()
    );

    // Step 1: Perform Low Degree Extension
    let extension_factor = 4; // Extend 8 steps to 32 steps
    let extended_trace = extend_trace(&trace, field, extension_factor);

    // Step 2: Commit to the EXTENDED trace (not the original)
    let mut flat_extended_trace = Vec::new();
    for row in &extended_trace {
        for element in row {
            flat_extended_trace.push(*element);
        }
    }

    // Build Merkle tree on extended trace
    let mut tree = MerkleTree::new();
    tree.build(&flat_extended_trace);

    let commitment = tree.root().unwrap();
    println!("   ✅ Extended trace committed: {}", commitment);

    // Create constraint polynomial
    let (constraint_poly, eval_domain) = create_fibonacci_constraint_poly(&trace, field);

    // Create empty sampling data (will be filled by verifier)
    let sampling_data = SamplingData {
        sample_points: Vec::new(),
        sample_values: Vec::new(),
        constraint_values: Vec::new(),
        merkle_proofs: Vec::new(),
    };

    StarkProof {
        trace_commitment: commitment,
        trace,
        field,
        constraint_poly,
        eval_domain,
        sampling_data,
    }
}

/// Generate Merkle proofs for sample points (prover's job)
pub fn generate_merkle_proofs(
    extended_trace: &[Vec<FiniteFieldElement>],
    sample_points: &[usize],
) -> Vec<Vec<i128>> {
    println!(
        "🌳 Prover generating Merkle proofs for {} sample points...",
        sample_points.len()
    );

    // Flatten the extended trace for Merkle tree
    let mut flat_extended_trace = Vec::new();
    for row in extended_trace {
        for element in row {
            flat_extended_trace.push(*element);
        }
    }

    // Build Merkle tree
    let mut tree = MerkleTree::new();
    tree.build(&flat_extended_trace);

    // Generate proofs for each sample point
    let mut merkle_proofs = Vec::new();
    for &sample_point in sample_points {
        if let Some(proof) = tree.get_merkle_proof(sample_point) {
            merkle_proofs.push(proof);
            println!(
                "   ✅ Generated Merkle proof for sample point {}",
                sample_point
            );
        } else {
            println!(
                "   ❌ Failed to generate Merkle proof for sample point {}",
                sample_point
            );
            merkle_proofs.push(Vec::new()); // Empty proof as fallback
        }
    }

    println!("   ✅ Generated {} Merkle proofs", merkle_proofs.len());
    merkle_proofs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::fibonacci;
    use crate::verifier::verify_proof;

    #[test]
    fn test_fibonacci_prover() {
        // Generate a small Fibonacci trace
        let trace = fibonacci::generate_fibonacci_trace(5, 1, 1);
        let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);

        // Generate proof
        let proof = prove_fibonacci(trace, field);

        // Verify proof using verifier
        let is_valid = verify_proof(&proof);

        assert!(is_valid, "Fibonacci proof should be valid");
    }
}
