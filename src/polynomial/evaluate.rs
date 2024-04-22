use super::polynomial::Polynomial;

impl Polynomial {
    pub fn evaluate(&self, x: i128) -> i128 {
        let mut result = 0_i128;
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result += coeff * x.pow(i as u32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation_none() {
        // f(X) = 0
        test_polynomial_eval([].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([].to_vec(), 5_i128, 0_i128);
        test_polynomial_eval([].to_vec(), -5_i128, 0_i128);
    }

    #[test]
    fn evaluation_single() {
        // f(X) = a
        test_polynomial_eval([0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128].to_vec(), 5_i128, 0_i128);
        test_polynomial_eval([0_i128].to_vec(), 5_i128, 0_i128);

        test_polynomial_eval([1_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), 1_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), 999_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), -999_i128, 1_i128);

        test_polynomial_eval([4_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128].to_vec(), 1_i128, 4_i128);
        test_polynomial_eval([4_i128].to_vec(), 999_i128, 4_i128);
    }

    #[test]
    fn evaluation_double() {
        // f(x) = ax + b

        // f(x) = 0x + 0
        test_polynomial_eval([0_i128, 0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128].to_vec(), 4_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128].to_vec(), -4_i128, 0_i128);

        // f(x) = 0x + 1
        test_polynomial_eval([1_i128, 0_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128].to_vec(), 4_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128].to_vec(), -4_i128, 1_i128);

        // f(x) = 2x + 4
        test_polynomial_eval([4_i128, 2_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128].to_vec(), 4_i128, 12_i128);
        test_polynomial_eval([4_i128, 2_i128].to_vec(), -4_i128, -4_i128);
    }

    #[test]
    fn evaluation_triple() {
        // f(x) = ax^2 + bx + c

        // f(x) = 0x^2 + 0x + 0
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), 4_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), -4_i128, 0_i128);

        // f(x) = 0x^2 + 0x + 1
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), 4_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), -4_i128, 1_i128);

        // f(x) = 0x^2 + 2x + 4
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), 4_i128, 12_i128);
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), -4_i128, -4_i128);

        // f(x) = 3x^2 + 2x + 4
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), 4_i128, 60_i128); // 48 + 8 + 4
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), -4_i128, 44_i128);
        // 48 - 8 + 4
    }

    fn test_polynomial_eval(coeffs: Vec<i128>, value: i128, expected_result: i128) {
        let pol: Polynomial = Polynomial::new(coeffs);
        assert_eq!(pol.evaluate(value), expected_result);
    }
}
