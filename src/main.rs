use stark_from_zero::{
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

    // Generate STARK proof (without sample data)
    let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
    let mut proof = prove_fibonacci(trace, field);

    // Verifier generates sample points via Fiat–Shamir
    println!("\n🔍 STARK Verification:");
    let extended_trace_size = 32; // 8 * 4 extension factor
    let leaf_count = extended_trace_size; // matches Merkle leaves for this setup
    let sample_points = derive_sample_points_from_commitment(proof.trace_commitment, leaf_count, 5);

    // Prover generates Merkle proofs for the sample points
    let extended_trace = extend_trace(&proof.trace, proof.field, 4);
    let merkle_proofs = generate_merkle_proofs(&extended_trace, &sample_points);

    // Prover provides sample values and Merkle proofs
    let mut sample_values = Vec::new();
    let mut constraint_values = Vec::new();

    for &sample_point in &sample_points {
        let mut values = Vec::new();
        for col in 0..extended_trace.len() {
            values.push(extended_trace[col][sample_point]);
        }
        sample_values.push(values);

        // Compute constraint value at this point
        let extended_eval_domain = EvaluationDomain::new_linear(proof.field, extended_trace_size);
        let point = extended_eval_domain.element(sample_point);
        let constraint_value = proof.constraint_poly.evaluate(point);
        constraint_values.push(constraint_value);
    }

    // Update proof with sample data
    proof.sampling_data.sample_points = sample_points;
    proof.sampling_data.sample_values = sample_values;
    proof.sampling_data.constraint_values = constraint_values;
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
