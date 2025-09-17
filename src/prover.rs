use crate::constants::EXTENSION_FACTOR;
use crate::evaluation_domain::EvaluationDomain;
use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::fri::fold_once;
use crate::merkle_tree::MerkleTree;
use crate::polynomial::interpolate::lagrange_interpolation;
use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;
use crate::verifier::{derive_fri_betas_from_commitment, SamplingData, StarkProof};

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
        let residual = f_n - f_n_minus_1 - f_n_minus_2;
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

/// Create the vanishing polynomial Z_H(x) = ∏(x - a_i) for domain H
fn create_vanishing_polynomial(domain: &EvaluationDomain) -> Polynomial {
    println!("🔧 Creating vanishing polynomial...");

    let mut result = Polynomial::new(vec![1]); // Start with 1

    for &point in &domain.points {
        // Multiply by (x - point) = [negated_point, 1]
        let negated_point = point.negate();
        let linear_factor = Polynomial::new_ff(vec![
            negated_point,
            FiniteFieldElement::new_fielded(1, point.field),
        ]);
        result = result.multiply(&linear_factor);
    }

    println!(
        "   ✅ Vanishing polynomial created (degree: {})",
        result.degree()
    );
    result
}

/// Compute quotient polynomial Q(x) = C(x) / Z_H(x)
/// This should be a low-degree polynomial if constraints are satisfied
fn create_quotient_polynomial(
    constraint_poly: &Polynomial,
    vanishing_poly: &Polynomial,
) -> Polynomial {
    println!("🔧 Creating quotient polynomial Q(x) = C(x) / Z_H(x)...");

    // If constraint polynomial is zero, quotient is zero
    if constraint_poly.degree() == 0
        && constraint_poly.coefficients.len() > 0
        && constraint_poly.coefficients[0].is_zero()
    {
        println!("   ✅ Constraint polynomial is zero, quotient is zero");
        return Polynomial::new(vec![0]);
    }

    // If constraint polynomial has lower degree than vanishing polynomial,
    // the quotient is zero and remainder is the constraint polynomial
    if constraint_poly.degree() < vanishing_poly.degree() {
        println!("   ✅ Constraint polynomial has lower degree than vanishing polynomial, quotient is zero");
        return Polynomial::new(vec![0]);
    }

    // Perform polynomial division: C(x) = Q(x) * Z_H(x) + R(x)
    let (quotient, remainder) = constraint_poly.div(vanishing_poly);

    // In a valid STARK, the remainder should be zero (or very small)
    if remainder.degree() > 0
        || (remainder.coefficients.len() > 0 && !remainder.coefficients[0].is_zero())
    {
        println!(
            "   ⚠️  Non-zero remainder in quotient computation: {}",
            remainder
        );
    } else {
        println!("   ✅ Quotient polynomial created with zero remainder");
    }

    println!(
        "   ✅ Quotient polynomial created (degree: {})",
        quotient.degree()
    );
    quotient
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
    let extension_factor = EXTENSION_FACTOR; // Extend trace by constant factor
    let extended_trace = extend_trace(&trace, field, extension_factor);

    // Step 2: Commit to the EXTENDED trace (row-leaf hashing)
    // Build leaves per row by hashing all column values together
    let extended_size = extended_trace[0].len();
    let num_cols = extended_trace.len();
    let mut row_leaf_hashes: Vec<i128> = Vec::with_capacity(extended_size);
    for i in 0..extended_size {
        let mut acc: i128 = 0;
        for c in 0..num_cols {
            let h = extended_trace[c][i].hash();
            acc = crate::merkle_tree::hash_two_inputs(acc, h);
        }
        // Use the accumulated hash directly as the leaf hash
        row_leaf_hashes.push(acc);
    }

    // Build Merkle tree on row leaf hashes (pad internally)
    let mut tree = MerkleTree::new();
    tree.build_from_hashes(&row_leaf_hashes);

    let commitment = tree.root().unwrap();
    println!("   ✅ Extended trace committed: {}", commitment);

    // Create a composition polynomial over original domain from the original trace
    let (composition_poly, eval_domain) = create_fibonacci_constraint_poly(&trace, field);

    // Create vanishing polynomial and quotient polynomial
    let vanishing_poly = create_vanishing_polynomial(&eval_domain);
    let quotient_poly = create_quotient_polynomial(&composition_poly, &vanishing_poly);

    // FRI: fold evaluations. Pad evaluations to Merkle leaf_count
    let mut fri_layers: Vec<Vec<FiniteFieldElement>> = Vec::new();
    let leaf_count = tree.leaf_count();
    let mut eval_leaves: Vec<FiniteFieldElement> = Vec::new();
    // Use a single combined evaluation per row: take, for simplicity, the last column F(n)
    for i in 0..extended_size {
        eval_leaves.push(extended_trace[num_cols - 1][i]);
    }
    if eval_leaves.len() < leaf_count {
        eval_leaves.resize(leaf_count, FiniteFieldElement::ZERO);
    }
    fri_layers.push(eval_leaves.clone());

    // Educational fixed betas (in practice via Fiat–Shamir)
    // Derive FRI betas via Fiat–Shamir from the Merkle root
    let fri_betas = derive_fri_betas_from_commitment(commitment, 2);
    let mut cur = eval_leaves;
    for &beta in &fri_betas {
        cur = fold_once(&cur, beta);
        fri_layers.push(cur.clone());
        if cur.len() <= 1 {
            break;
        }
    }

    // Create empty sampling data (will be filled by verifier)
    let sampling_data = SamplingData {
        sample_points: Vec::new(),
        sample_values: Vec::new(),
        constraint_values: Vec::new(),
        merkle_proofs: Vec::new(),
    };

    StarkProof {
        trace_commitment: commitment,
        trace_size: trace.num_rows(),
        field,
        eval_domain,
        sampling_data,
        fri_layers,
        fri_betas,
        composition_poly,
        quotient_poly,
    }
}

/// Generate Merkle proofs for sample points
pub fn generate_merkle_proofs(
    extended_trace: &[Vec<FiniteFieldElement>],
    sample_points: &[usize],
) -> Vec<Vec<i128>> {
    println!(
        "🌳 Prover generating Merkle proofs for {} sample points...",
        sample_points.len()
    );

    // Build the same row-leaf Merkle tree as in prove_fibonacci
    let extended_size = extended_trace[0].len();
    let num_cols = extended_trace.len();
    let mut row_leaf_hashes: Vec<i128> = Vec::with_capacity(extended_size);
    for i in 0..extended_size {
        let mut acc: i128 = 0;
        for c in 0..num_cols {
            let h = extended_trace[c][i].hash();
            acc = crate::merkle_tree::hash_two_inputs(acc, h);
        }
        row_leaf_hashes.push(acc);
    }

    let mut tree = MerkleTree::new();
    tree.build_from_hashes(&row_leaf_hashes);

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
    use crate::constants::DEFAULT_FIELD_SIZE;
    use crate::trace::fibonacci;
    use crate::verifier::verify_proof;

    #[test]
    fn test_fibonacci_prover() {
        // Generate a small Fibonacci trace
        let trace = fibonacci::generate_fibonacci_trace(5, 1, 1);
        let field = FiniteField::new(DEFAULT_FIELD_SIZE);

        // Generate proof
        let mut proof = prove_fibonacci(trace.clone(), field);

        // Set up sampling data like in main
        let extension_factor = EXTENSION_FACTOR;
        let extended_trace = super::extend_trace(&trace, proof.field, extension_factor);
        let extended_trace_size = proof.trace_size * extension_factor;

        let sample_points = crate::verifier::generate_sample_points(extended_trace_size, 5);
        // Generate Merkle proofs by rebuilding the same tree (for testing only)
        let merkle_proofs = super::generate_merkle_proofs(&extended_trace, &sample_points);

        // Collect sample values (constraint values will be derived by verifier)
        let mut sample_values = Vec::new();
        for &sample_point in &sample_points {
            let mut values_at_point = Vec::new();
            for col in 0..extended_trace.len() {
                values_at_point.push(extended_trace[col][sample_point]);
            }
            sample_values.push(values_at_point);
        }

        proof.sampling_data.sample_points = sample_points;
        proof.sampling_data.sample_values = sample_values;
        proof.sampling_data.constraint_values = Vec::new(); // Verifier will derive these
        proof.sampling_data.merkle_proofs = merkle_proofs;

        // Verify proof using verifier
        let is_valid = verify_proof(&proof);

        assert!(is_valid, "Fibonacci proof should be valid");
    }

    #[test]
    fn test_vanishing_polynomial() {
        let field = FiniteField::new(DEFAULT_FIELD_SIZE);
        let domain = EvaluationDomain::new_linear(field, 3); // points: 0, 1, 2

        let vanishing_poly = create_vanishing_polynomial(&domain);

        // Vanishing polynomial should be zero at all domain points
        for i in 0..domain.size() {
            let point = domain.element(i);
            let value = vanishing_poly.evaluate(point);
            assert_eq!(
                value.value, 0,
                "Vanishing polynomial should be zero at domain point {}",
                i
            );
        }

        // Vanishing polynomial should be non-zero outside the domain
        let outside_point = FiniteFieldElement::new_fielded(3, field);
        let outside_value = vanishing_poly.evaluate(outside_point);
        assert_ne!(
            outside_value.value, 0,
            "Vanishing polynomial should be non-zero outside domain"
        );
    }

    #[test]
    fn test_quotient_polynomial() {
        let field = FiniteField::new(DEFAULT_FIELD_SIZE);
        let domain = EvaluationDomain::new_linear(field, 3); // points: 0, 1, 2

        // Create a simple constraint polynomial that's zero at domain points
        // C(x) = x(x-1)(x-2) = x^3 - 3x^2 + 2x
        let constraint_poly = Polynomial::new(vec![0, 2, -3, 1]);

        let vanishing_poly = create_vanishing_polynomial(&domain);
        let quotient_poly = create_quotient_polynomial(&constraint_poly, &vanishing_poly);

        // The quotient should be a constant (degree 0) since C(x) = (x-0)(x-1)(x-2) * 1
        assert_eq!(
            quotient_poly.degree(),
            0,
            "Quotient polynomial should be constant"
        );
        assert_eq!(
            quotient_poly.coefficients[0].value, 1,
            "Quotient should be 1"
        );

        // Verify: C(x) = Q(x) * Z_H(x) at a few test points
        let test_points = vec![0, 1, 2, 3, 4];
        for &i in &test_points {
            let point = FiniteFieldElement::new_fielded(i, field);
            let c_value = constraint_poly.evaluate(point);
            let z_value = vanishing_poly.evaluate(point);
            let q_value = quotient_poly.evaluate(point);
            let expected = q_value.multiply(z_value);

            assert_eq!(
                c_value.value, expected.value,
                "C({}) = {} should equal Q({}) * Z_H({}) = {}",
                i, c_value.value, i, i, expected.value
            );
        }
    }
}
