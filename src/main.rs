use stark_from_zero::{
    constants::{DEFAULT_FIELD_SIZE, EXTENSION_FACTOR},
    evaluation_domain::EvaluationDomain,
    finite_field::{FiniteField, FiniteFieldElement},
    prover::{extend_trace, generate_merkle_proofs, prove_fibonacci},
    trace::fibonacci,
    verifier::{derive_sample_points_from_commitment, verify_proof},
};

fn main() {
    println!("=== Simple STARK Prover Example ===");

    // Generate a Fibonacci trace
    let trace = fibonacci::generate_fibonacci_trace(8, 1, 1);

    println!("\n📊 Fibonacci trace (8 steps):");
    println!("Step | F(n-2) | F(n-1) | F(n)");
    println!("-----|--------|--------|-----");

    for step in 0..trace.num_rows() {
        let f_n_minus_2 = trace.get(step, 0).unwrap();
        let f_n_minus_1 = trace.get(step, 1).unwrap();
        let f_n = trace.get(step, 2).unwrap();
        println!(
            "  {}  |   {}   |   {}   |  {}",
            step, f_n_minus_2, f_n_minus_1, f_n
        );
    }

    // Generate STARK proof
    let field = FiniteField::new(DEFAULT_FIELD_SIZE);
    let mut proof = prove_fibonacci(trace.clone(), field);

    // Verifier generates sample points via Fiat–Shamir
    println!("\n🔍 STARK Verification:");
    let extended_trace_size = 8 * EXTENSION_FACTOR; // matches Merkle leaves for this setup
    let leaf_count = extended_trace_size; // matches Merkle leaves for this setup
    let sample_points = derive_sample_points_from_commitment(proof.trace_commitment, leaf_count, 5);

    // Prover generates Merkle proofs for the sample points
    // Note: In a real STARK, the prover would have already computed this during proof generation
    // For this educational example, we recompute it
    let extended_trace = extend_trace(&trace, proof.field, EXTENSION_FACTOR);
    let merkle_proofs = generate_merkle_proofs(&extended_trace, &sample_points);

    // Prover provides sample values and Merkle proofs
    let mut sample_values = Vec::new();

    // Get sample values from the extended trace
    for &sample_point in &sample_points {
        let mut values = Vec::new();
        for col in 0..extended_trace.len() {
            values.push(extended_trace[col][sample_point]);
        }
        sample_values.push(values);
    }

    // Update proof with sample data
    proof.sampling_data.sample_points = sample_points;
    proof.sampling_data.sample_values = sample_values;
    proof.sampling_data.constraint_values = Vec::new(); // Verifier will derive these
    proof.sampling_data.merkle_proofs = merkle_proofs;

    // Verify the proof
    let is_valid = verify_proof(&proof);

    println!("\n🎯 STARK Proof Result:");
    if is_valid {
        println!("   ✅ Proof is VALID - Fibonacci computation is correct!");
    } else {
        println!("   ❌ Proof is INVALID - Computation has errors!");
    }
}
