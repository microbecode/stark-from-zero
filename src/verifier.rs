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

/// Verify random sampling with specific sample points
pub fn verify_random_sampling_with_points(proof: &StarkProof, sample_points: &[usize]) -> bool {
    println!(
        "🎲 Verifying random sampling with {} sample points...",
        sample_points.len()
    );

    let mut valid = true;

    for (i, &sample_point) in sample_points.iter().enumerate() {
        // Evaluate constraint polynomial at this point
        let extended_eval_domain =
            EvaluationDomain::new_linear(proof.field, proof.extended_trace.len());
        let point = extended_eval_domain.element(sample_point);
        let constraint_value = proof.constraint_poly.evaluate(point);

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

/// Generate random sample points (verifier's job)
pub fn generate_sample_points(extended_trace_size: usize, num_samples: usize) -> Vec<usize> {
    println!("🎲 Verifier generating random sample points...");

    let mut sample_points = Vec::new();

    // Simple PRNG for educational purposes
    // In a real STARK, this would use Fiat-Shamir with the proof commitment
    let mut rng_state = 12345u64; // Simple seed
    for _ in 0..num_samples {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let sample_point = (rng_state as usize) % extended_trace_size;
        sample_points.push(sample_point);
        println!("   Generated sample point: {}", sample_point);
    }

    println!("   ✅ Generated {} random sample points", num_samples);
    sample_points
}

/// Verify the entire STARK proof
pub fn verify_proof(proof: &StarkProof) -> bool {
    println!("🔍 Verifying STARK proof...");

    // Generate random sample points (verifier's responsibility)
    let sample_points = generate_sample_points(proof.extended_trace.len(), 5);

    // Verify using the generated sample points
    let is_valid = verify_random_sampling_with_points(proof, &sample_points);

    if is_valid {
        println!("   ✅ STARK proof is VALID!");
    } else {
        println!("   ❌ STARK proof is INVALID!");
    }

    is_valid
}
