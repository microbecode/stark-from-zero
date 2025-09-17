use crate::constants::{DEFAULT_FIELD_SIZE, EXTENSION_FACTOR};
use crate::evaluation_domain::EvaluationDomain;
use crate::finite_field::FiniteFieldElement;
use crate::polynomial::polynomial::Polynomial;
use crate::{fiat_shamir::Transcript, finite_field::FiniteField};

/// Random sampling data for verification
pub struct SamplingData {
    /// Random points chosen by verifier
    pub sample_points: Vec<usize>,
    /// Values at those points (provided by prover)
    pub sample_values: Vec<Vec<FiniteFieldElement>>,
    /// Constraint polynomial values at sample points
    pub constraint_values: Vec<FiniteFieldElement>,
    /// Merkle proofs for the sample points
    pub merkle_proofs: Vec<Vec<i128>>,
}

/// STARK proof structure (shared between prover and verifier)
pub struct StarkProof {
    /// Merkle root of the extended trace
    pub trace_commitment: i128,
    /// Original trace size (number of rows)
    pub trace_size: usize,
    /// The field used
    pub field: crate::finite_field::FiniteField,
    /// Evaluation domain for the extended trace
    pub eval_domain: EvaluationDomain,
    /// Random sampling points and values
    pub sampling_data: SamplingData,
    /// FRI folding layers over the same leaves as Merkle (padded to power-of-two)
    pub fri_layers: Vec<Vec<FiniteFieldElement>>,
    /// Folding betas used per round (educational, fixed for now)
    pub fri_betas: Vec<FiniteFieldElement>,
    /// Composition polynomial C(x) = f(x+2) - f(x+1) - f(x) over original domain
    pub composition_poly: Polynomial,
    /// Quotient polynomial Q(x) = C(x) / Z_H(x) - should be low degree
    pub quotient_poly: Polynomial,
}

/// Verify constraints using composition polynomial provided by prover
fn verify_fibonacci_constraints(
    sample_points: &[usize],
    trace_size: usize,
    field: FiniteField,
    composition_poly: &Polynomial,
) -> bool {
    println!("🔧 Verifying Fibonacci constraints using composition polynomial...");

    let mut valid = true;
    let mut checked_count = 0;

    // Check composition polynomial at all sampled points
    for (i, &sample_point) in sample_points.iter().enumerate() {
        // Evaluate composition polynomial at this point
        let extended_eval_domain =
            EvaluationDomain::new_linear(field, trace_size * EXTENSION_FACTOR);
        let point = extended_eval_domain.element(sample_point);
        let constraint_value = composition_poly.evaluate(point);

        if !constraint_value.is_zero() {
            println!(
                "   ❌ Sample {} (point {}): C({}) = {} (should be 0)",
                i, sample_point, sample_point, constraint_value.value
            );
            valid = false;
        } else {
            println!(
                "   ✅ Sample {} (point {}): C({}) = 0",
                i, sample_point, sample_point
            );
        }
        checked_count += 1;
    }

    if valid {
        println!(
            "   ✅ All Fibonacci constraints verified! (checked {} points)",
            checked_count
        );
    } else {
        println!("   ❌ Some Fibonacci constraints failed verification!");
    }

    valid
}

/// Verify quotient polynomial at sample points
/// This checks that Q(x) = C(x) / Z_H(x) is well-defined and low-degree
fn verify_quotient_polynomial(
    sample_points: &[usize],
    trace_size: usize,
    field: FiniteField,
    composition_poly: &Polynomial,
    quotient_poly: &Polynomial,
) -> bool {
    println!("🔧 Verifying quotient polynomial Q(x) = C(x) / Z_H(x)...");

    let mut valid = true;
    let mut checked_count = 0;

    // Create the original domain for vanishing polynomial
    let original_domain = EvaluationDomain::new_linear(field, trace_size);

    // Check quotient polynomial at all sampled points
    for (i, &sample_point) in sample_points.iter().enumerate() {
        let extended_eval_domain =
            EvaluationDomain::new_linear(field, trace_size * EXTENSION_FACTOR);
        let point = extended_eval_domain.element(sample_point);

        // Evaluate composition polynomial C(x)
        let constraint_value = composition_poly.evaluate(point);

        // Evaluate vanishing polynomial Z_H(x) at the point
        let vanishing_value = original_domain.evaluate_vanishing(point);

        // Evaluate quotient polynomial Q(x)
        let quotient_value = quotient_poly.evaluate(point);

        // Check: C(x) should equal Q(x) * Z_H(x)
        let expected_constraint = quotient_value.multiply(vanishing_value);

        if constraint_value != expected_constraint {
            println!(
                "   ❌ Sample {} (point {}): C({}) = {}, Q({}) * Z_H({}) = {} (should be equal)",
                i,
                sample_point,
                sample_point,
                constraint_value.value,
                sample_point,
                sample_point,
                expected_constraint.value
            );
            valid = false;
        } else {
            println!(
                "   ✅ Sample {} (point {}): Q({}) * Z_H({}) = C({}) = {}",
                i, sample_point, sample_point, sample_point, sample_point, constraint_value.value
            );
        }
        checked_count += 1;
    }

    if valid {
        println!(
            "   ✅ Quotient polynomial verification passed! (checked {} points)",
            checked_count
        );
    } else {
        println!("   ❌ Quotient polynomial verification failed!");
    }

    valid
}

/// Verify random sampling: check that constraint polynomial is zero at sample points
pub fn verify_random_sampling(proof: &StarkProof) -> bool {
    println!("🎲 Verifying random sampling...");

    // Verify constraints using composition polynomial
    let constraint_valid = verify_fibonacci_constraints(
        &proof.sampling_data.sample_points,
        proof.trace_size,
        proof.field,
        &proof.composition_poly,
    );

    // Verify quotient polynomial
    let quotient_valid = verify_quotient_polynomial(
        &proof.sampling_data.sample_points,
        proof.trace_size,
        proof.field,
        &proof.composition_poly,
        &proof.quotient_poly,
    );

    constraint_valid && quotient_valid
}

/// Verify Merkle proofs for sample points (verifier only verifies, doesn't reconstruct)
pub fn verify_merkle_proofs(proof: &StarkProof) -> bool {
    println!("🌳 Verifying Merkle proofs for sample points...");

    let mut valid = true;

    for (i, (&sample_point, merkle_proof)) in proof
        .sampling_data
        .sample_points
        .iter()
        .zip(proof.sampling_data.merkle_proofs.iter())
        .enumerate()
    {
        // Get the sample values provided by prover
        let sample_values = &proof.sampling_data.sample_values[i];

        if merkle_proof.is_empty() {
            println!(
                "   ❌ Sample {} (point {}): No Merkle proof provided",
                i, sample_point
            );
            valid = false;
            continue;
        }

        // Compute row-leaf hash from all column values (must match prover logic)
        let mut acc: i128 = 0;
        for v in sample_values.iter() {
            let h = v.hash();
            acc = crate::merkle_tree::hash_two_inputs(acc, h);
        }
        // Tree was built from leaf hashes = accumulated_row_hash directly (no extra hash)
        let leaf_hash = acc;
        let mut current_hash = leaf_hash;

        // Reconstruct root by following the proof path
        for sibling in &merkle_proof[..merkle_proof.len() - 1] {
            current_hash = crate::merkle_tree::hash_two_inputs(current_hash, *sibling);
        }

        // Check if reconstructed root matches committed root
        if current_hash == proof.trace_commitment {
            println!(
                "   ✅ Sample {} (point {}): Merkle proof verified",
                i, sample_point
            );
            println!(
                "      Values: {:?}",
                sample_values.iter().map(|v| v.value).collect::<Vec<_>>()
            );
        } else {
            println!(
                "   ❌ Sample {} (point {}): Merkle proof failed",
                i, sample_point
            );
            println!(
                "      Expected root: {}, Got: {}",
                proof.trace_commitment, current_hash
            );
            println!(
                "      Leaf hash: {}, Values: {:?}",
                leaf_hash,
                sample_values.iter().map(|v| v.value).collect::<Vec<_>>()
            );
            valid = false;
        }
    }

    if valid {
        println!("   ✅ All Merkle proofs verified!");
    } else {
        println!("   ❌ Some Merkle proofs failed verification!");
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

/// Derive sample points using Fiat–Shamir from the commitment and leaf count
pub fn derive_sample_points_from_commitment(
    commitment: i128,
    leaf_count: usize,
    num_samples: usize,
) -> Vec<usize> {
    println!("🎲 Deriving sample points via Fiat–Shamir...");
    let field = FiniteField::new(DEFAULT_FIELD_SIZE);
    let mut t = Transcript::new();
    t.absorb_i128(commitment);
    t.absorb_i128(leaf_count as i128);

    let mut points = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        let c = t.challenge(field);
        let idx = ((c.value % (leaf_count as i128)) + (leaf_count as i128)) % (leaf_count as i128);
        points.push(idx as usize);
    }
    println!("   ✅ Derived {} sample points", num_samples);
    points
}

/// Derive FRI betas using Fiat–Shamir from the commitment
pub fn derive_fri_betas_from_commitment(
    commitment: i128,
    num_rounds: usize,
) -> Vec<FiniteFieldElement> {
    println!("🧪 Deriving FRI betas via Fiat–Shamir...");
    let field = FiniteField::new(DEFAULT_FIELD_SIZE);
    let mut t = Transcript::new();
    t.absorb_i128(commitment);

    let mut betas = Vec::with_capacity(num_rounds);
    for _ in 0..num_rounds {
        let beta = t.challenge(field);
        betas.push(beta);
    }
    println!("   ✅ Derived {} betas", num_rounds);
    betas
}

/// Verify the entire STARK proof
pub fn verify_proof(proof: &StarkProof) -> bool {
    println!("🔍 Verifying STARK proof...");

    // Check if we have sample data
    if proof.sampling_data.sample_points.is_empty() {
        println!("   ❌ No sample data provided by prover!");
        return false;
    }

    // Step 1: Verify Merkle proofs for sample points
    let merkle_valid = verify_merkle_proofs(proof);

    // Step 2: Verify constraint polynomial at sample points
    let constraint_valid = verify_random_sampling(proof);

    // Both verifications must pass
    let is_valid = merkle_valid && constraint_valid;

    if is_valid {
        println!("   ✅ STARK proof is VALID!");
    } else {
        println!("   ❌ STARK proof is INVALID!");
        if !merkle_valid {
            println!("   ❌ Merkle proof verification failed!");
        }
        if !constraint_valid {
            println!("   ❌ Constraint verification failed!");
        }
    }

    is_valid
}
