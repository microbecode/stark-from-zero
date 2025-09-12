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
}

/// Step 1: Basic prover that just commits to the trace
pub fn prove_fibonacci(trace: Trace, field: FiniteField) -> StarkProof {
    println!("🔍 Starting STARK proof generation...");
    println!(
        "   Trace size: {} rows × {} columns",
        trace.num_rows(),
        trace.num_columns()
    );

    // For now, let's just commit to the original trace (no LDE yet)
    let ff_trace = trace.to_finite_field_elements(field);

    // Flatten the trace for Merkle tree (we'll improve this later)
    let mut flat_trace = Vec::new();
    for row in &ff_trace {
        for element in row {
            flat_trace.push(*element);
        }
    }

    // Build Merkle tree
    let mut tree = MerkleTree::new();
    tree.build(&flat_trace);

    let commitment = tree.root().unwrap();
    println!("   ✅ Trace committed: {}", commitment);

    StarkProof {
        trace_commitment: commitment,
        trace,
        field,
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
