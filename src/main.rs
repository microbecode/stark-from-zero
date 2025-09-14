use stark_from_zero::{
    finite_field::{FiniteField, FiniteFieldElement},
    prover::{prove_fibonacci, verify_constraint_polynomial, verify_fibonacci_constraints},
    trace::{fibonacci, Trace},
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
    let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
    let proof = prove_fibonacci(trace, field);

    // Verify the proof using both methods
    println!("\n🔍 Verification Methods:");

    // Method 1: Step-by-step verification
    let step_by_step_valid = verify_fibonacci_constraints(&proof);

    // Method 2: Polynomial constraint verification
    let polynomial_valid = verify_constraint_polynomial(&proof);

    println!("\n🎯 STARK Proof Result:");
    if step_by_step_valid && polynomial_valid {
        println!("   ✅ Proof is VALID - Fibonacci computation is correct!");
        println!("   ✅ Both verification methods passed!");
    } else {
        println!("   ❌ Proof is INVALID - Computation has errors!");
        if !step_by_step_valid {
            println!("   ❌ Step-by-step verification failed!");
        }
        if !polynomial_valid {
            println!("   ❌ Polynomial constraint verification failed!");
        }
    }
}
