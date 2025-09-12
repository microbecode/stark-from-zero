use stark_from_zero::{
    finite_field::{FiniteField, FiniteFieldElement},
    trace::{fibonacci, Trace},
    //   prover::prove,
};

fn main() {
    println!("=== STARK Trace Example ===");

    // Generate a Fibonacci trace
    let trace = fibonacci::generate_fibonacci_trace(8, 1, 1);

    println!("Fibonacci trace (8 steps):");
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

    println!(
        "\nTrace dimensions: {} rows × {} columns",
        trace.num_rows(),
        trace.num_columns()
    );

    // Show how to access specific values
    println!("\nF(5) = {}", trace.get(5, 2).unwrap());
    println!("F(6) = {}", trace.get(6, 2).unwrap());

    // Convert to finite field elements
    let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
    let ff_trace = trace.to_finite_field_elements(field);
    println!(
        "\nFirst row in finite field: {:?}",
        ff_trace[0].iter().map(|e| e.value).collect::<Vec<_>>()
    );
}
