use crate::evaluation_domain::EvaluationDomain;
use crate::finite_field::FiniteFieldElement;
use crate::polynomial::polynomial::Polynomial;

/// Random sampling data for verification
pub struct SamplingData {
    /// Random points chosen by verifier
    pub sample_points: Vec<usize>,
    /// Values at those points (provided by prover)
    pub sample_values: Vec<Vec<FiniteFieldElement>>,
    /// Constraint polynomial values at sample points
    pub constraint_values: Vec<FiniteFieldElement>,
}

/// STARK proof structure (shared between prover and verifier)
pub struct StarkProof {
    /// Merkle root of the extended trace
    pub trace_commitment: i128,
    /// The original trace
    pub trace: crate::trace::Trace,
    /// The field used
    pub field: crate::finite_field::FiniteField,
    /// The extended trace (LDE)
    pub extended_trace: Vec<Vec<FiniteFieldElement>>,
    /// Constraint polynomial: C(x) = F(x) - F(x-1) - F(x-2)
    pub constraint_poly: Polynomial,
    /// Evaluation domain for the extended trace
    pub eval_domain: EvaluationDomain,
    /// Random sampling points and values
    pub sampling_data: SamplingData,
}

/// Verify random sampling: check that constraint polynomial is zero at sample points
pub fn verify_random_sampling(proof: &StarkProof) -> bool {
    println!("🎲 Verifying random sampling...");

    let sampling_data = &proof.sampling_data;
    let mut valid = true;

    println!(
        "   Checking {} random samples...",
        sampling_data.sample_points.len()
    );

    for (i, &sample_point) in sampling_data.sample_points.iter().enumerate() {
        let constraint_value = sampling_data.constraint_values[i];

        if constraint_value.value != 0 {
            println!(
                "   ❌ Sample {} (point {}): constraint_value={} (should be 0)",
                i, sample_point, constraint_value.value
            );
            valid = false;
        } else {
            println!(
                "   ✅ Sample {} (point {}): constraint_value=0",
                i, sample_point
            );
        }
    }

    if valid {
        println!("   ✅ All random samples verified - constraint polynomial is zero!");
    } else {
        println!("   ❌ Some random samples failed verification!");
    }

    valid
}

/// Verify the entire STARK proof
pub fn verify_proof(proof: &StarkProof) -> bool {
    println!("🔍 Verifying STARK proof...");

    // For now, we only have random sampling verification
    // In a real STARK, this would include:
    // - Merkle proof verification
    // - FRI protocol verification
    // - Additional constraint checks

    let is_valid = verify_random_sampling(proof);

    if is_valid {
        println!("   ✅ STARK proof is VALID!");
    } else {
        println!("   ❌ STARK proof is INVALID!");
    }

    is_valid
}
