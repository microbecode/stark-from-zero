// This file is used for evaluating square fibonacci sequence, used in Stark 101 examples.
// All unit test values are from the videos https://starkware.co/stark-101/

use crate::number::modulo_multiply;

pub fn evaluate_sq_fibo(
    a: i128,
    b: i128,
    modulo: i128,
    results: &mut Vec<i128>,

    curr_depth: i128,
    max_depth: i128,
) {
    // -2 because the first two elements are already given initially
    if curr_depth < max_depth - 2 {
        let pow_a = modulo_multiply(a, a, modulo);
        let pow_b = modulo_multiply(b, b, modulo);
        let res = (pow_a + pow_b) % modulo;

        evaluate_sq_fibo(b, res, modulo, results, curr_depth + 1, max_depth);
    }

    results.push(b);
    if curr_depth == 0 {
        results.push(a);
        results.reverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::{finite_field::FiniteFieldElement, sq_fibo::evaluate_sq_fibo};

    #[test]
    fn sq_fibo_no_modulo() {
        let mut results = vec![];
        evaluate_sq_fibo(1, 3, i128::MAX, &mut results, 0, 6);
        assert_eq!(results, vec![1, 3, 10, 109, 11981, 143556242]);
    }

    #[test]
    fn sq_fibo_mini() {
        let mut results = vec![];
        evaluate_sq_fibo(1, 3, 7, &mut results, 0, 6);
        assert_eq!(results, vec![1, 3, 3, 4, 4, 4]);
    }

    #[test]
    fn sq_fibo_full() {
        let mut results = vec![];
        evaluate_sq_fibo(1, 3141592, 3221225473, &mut results, 0, 102);
        assert_eq!(results[1022], 2338775057);
    }
}
