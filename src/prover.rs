use crate::evaluation_domain::EvaluationDomain;
use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::merkle_tree::MerkleTree;
use crate::polynomial::interpolate::lagrange_interpolation;
use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;

/// Simple STARK proof for Fibonacci computation
pub struct StarkProof {
    /// Merkle root of the extended trace
    pub trace_commitment: i128,
    /// The original trace
    pub trace: Trace,
    /// The field used
    pub field: FiniteField,
    /// The extended trace (LDE)
    pub extended_trace: Vec<Vec<FiniteFieldElement>>,
    /// Constraint residuals on original trace: r[n] = F(n) - F(n-1) - F(n-2).
    /// This is only for debugging purposes.
    pub residuals: Vec<i128>,
    /// Constraint polynomial: C(x) = F(x) - F(x-1) - F(x-2)
    pub constraint_poly: Polynomial,
    /// Evaluation domain for the extended trace
    pub eval_domain: EvaluationDomain,
}

/// Low Degree Extension: Interpolate trace columns and evaluate at larger domain
fn extend_trace(
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

/// Compute residuals r[n] = F(n) - F(n-1) - F(n-2) over original trace
fn compute_fibonacci_residuals(trace: &Trace) -> Vec<i128> {
    let mut residuals = Vec::with_capacity(trace.num_rows());
    if trace.num_rows() == 0 {
        return residuals;
    }
    // For first two rows, define residual as 0 (no previous terms)
    residuals.push(0);
    if trace.num_rows() > 1 {
        residuals.push(0);
    }
    for step in 2..trace.num_rows() {
        let f_n_minus_2 = trace.get(step, 0).unwrap();
        let f_n_minus_1 = trace.get(step, 1).unwrap();
        let f_n = trace.get(step, 2).unwrap();
        residuals.push(f_n - (f_n_minus_1 + f_n_minus_2));
    }
    residuals
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

    // Compute simple residuals on the original (non-extended) trace
    let residuals = compute_fibonacci_residuals(&trace);
    let non_zero: usize = residuals.iter().filter(|v| **v != 0).count();
    println!(
        "   Constraint residuals: {} zeros, {} non-zeros",
        residuals.len() - non_zero,
        non_zero
    );

    // Create constraint polynomial
    let (constraint_poly, eval_domain) = create_fibonacci_constraint_poly(&trace, field);

    StarkProof {
        trace_commitment: commitment,
        trace,
        field,
        extended_trace,
        residuals,
        constraint_poly,
        eval_domain,
    }
}

/// Step 2: Verify that the trace follows Fibonacci rules
pub fn verify_fibonacci_constraints(proof: &StarkProof) -> bool {
    println!("🔍 Verifying Fibonacci constraints...");

    let trace = &proof.trace;
    let mut valid = true;

    // Check that F(n) = F(n-1) + F(n-2) for all steps after the first two
    for step in 2..trace.num_rows() {
        let f_n_minus_2 = trace.get(step, 0).unwrap();
        let f_n_minus_1 = trace.get(step, 1).unwrap();
        let f_n = trace.get(step, 2).unwrap();

        let expected_f_n = f_n_minus_1 + f_n_minus_2;

        if f_n != expected_f_n {
            println!(
                "   ❌ Constraint failed at step {}: F({}) = {} but expected {}",
                step, step, f_n, expected_f_n
            );
            valid = false;
        } else {
            println!(
                "   ✅ Step {}: F({}) = {} = F({}) + F({}) = {} + {}",
                step,
                step,
                f_n,
                step - 1,
                step - 2,
                f_n_minus_1,
                f_n_minus_2
            );
        }
    }

    if valid {
        println!("   ✅ All Fibonacci constraints verified!");
    } else {
        println!("   ❌ Some constraints failed!");
    }

    valid
}

/// Verify constraint polynomial: check that C(x) = 0 at all original trace points
/// In reality, we should not use this method because it is not efficient.
pub fn verify_constraint_polynomial(proof: &StarkProof) -> bool {
    println!("🔧 Verifying constraint polynomial...");

    let constraint_poly = &proof.constraint_poly;
    let eval_domain = &proof.eval_domain;
    let mut valid = true;

    // Check constraint polynomial at all original trace points
    for i in 0..proof.trace.num_rows() {
        let point = eval_domain.element(i);
        let constraint_value = constraint_poly.evaluate(point);

        if constraint_value.value != 0 {
            println!(
                "   ❌ Constraint polynomial non-zero at step {}: {}",
                i, constraint_value.value
            );
            valid = false;
        } else {
            println!("   ✅ Step {}: C({}) = 0", i, i);
        }
    }

    if valid {
        println!("   ✅ Constraint polynomial verified - all evaluations are zero!");
    } else {
        println!("   ❌ Constraint polynomial verification failed!");
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::fibonacci;

    #[test]
    fn test_fibonacci_prover() {
        // Generate a small Fibonacci trace
        let trace = fibonacci::generate_fibonacci_trace(5, 1, 1);
        let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);

        // Generate proof
        let proof = prove_fibonacci(trace, field);

        // Verify constraints
        let is_valid = verify_fibonacci_constraints(&proof);

        assert!(is_valid, "Fibonacci constraints should be valid");
    }
}
