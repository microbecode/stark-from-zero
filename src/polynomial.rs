pub struct Polynomial {
    coefficients: Vec<i64>,
}

impl Polynomial {
    /// Note that the coefficients are enumerated from the "right": the first entry is for constant value.
    /// The second entry is for coefficient * x, third entry for coefficient * x^2, etc
    pub fn new(coefficients: Vec<i64>) -> Self {
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: i64) -> i64 {
        let mut result = 0;
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
        test_polynomial([].to_vec(), 0_i64, 0_i64);
        test_polynomial([].to_vec(), 5_i64, 0_i64);
        test_polynomial([].to_vec(), -5_i64, 0_i64);
    }

    #[test]
    fn evaluation_single() {
        // f(X) = a
        test_polynomial([0_i64].to_vec(), 0_i64, 0_i64);
        test_polynomial([0_i64].to_vec(), 5_i64, 0_i64);
        test_polynomial([0_i64].to_vec(), 5_i64, 0_i64);

        test_polynomial([1_i64].to_vec(), 0_i64, 1_i64);
        test_polynomial([1_i64].to_vec(), 1_i64, 1_i64);
        test_polynomial([1_i64].to_vec(), 999_i64, 1_i64);
        test_polynomial([1_i64].to_vec(), -999_i64, 1_i64);

        test_polynomial([4_i64].to_vec(), 0_i64, 4_i64);
        test_polynomial([4_i64].to_vec(), 1_i64, 4_i64);
        test_polynomial([4_i64].to_vec(), 999_i64, 4_i64);
    }

    #[test]
    fn evaluation_double() {
        // f(x) = ax + b

        // f(x) = 0x + 0
        test_polynomial([0_i64, 0_i64].to_vec(), 0_i64, 0_i64);
        test_polynomial([0_i64, 0_i64].to_vec(), 4_i64, 0_i64);
        test_polynomial([0_i64, 0_i64].to_vec(), -4_i64, 0_i64);

        // f(x) = 0x + 1
        test_polynomial([1_i64, 0_i64].to_vec(), 0_i64, 1_i64);
        test_polynomial([1_i64, 0_i64].to_vec(), 4_i64, 1_i64);
        test_polynomial([1_i64, 0_i64].to_vec(), -4_i64, 1_i64);

        // f(x) = 2x + 4
        test_polynomial([4_i64, 2_i64].to_vec(), 0_i64, 4_i64);
        test_polynomial([4_i64, 2_i64].to_vec(), 4_i64, 12_i64);
        test_polynomial([4_i64, 2_i64].to_vec(), -4_i64, -4_i64);
    }

    #[test]
    fn evaluation_triple() {
        // f(x) = ax^2 + bx + c

        // f(x) = 0x^2 + 0x + 0
        test_polynomial([0_i64, 0_i64, 0_i64].to_vec(), 0_i64, 0_i64);
        test_polynomial([0_i64, 0_i64, 0_i64].to_vec(), 4_i64, 0_i64);
        test_polynomial([0_i64, 0_i64, 0_i64].to_vec(), -4_i64, 0_i64);

        // f(x) = 0x^2 + 0x + 1
        test_polynomial([1_i64, 0_i64, 0_i64].to_vec(), 0_i64, 1_i64);
        test_polynomial([1_i64, 0_i64, 0_i64].to_vec(), 4_i64, 1_i64);
        test_polynomial([1_i64, 0_i64, 0_i64].to_vec(), -4_i64, 1_i64);

        // f(x) = 0x^2 + 2x + 4
        test_polynomial([4_i64, 2_i64, 0_i64].to_vec(), 0_i64, 4_i64);
        test_polynomial([4_i64, 2_i64, 0_i64].to_vec(), 4_i64, 12_i64);
        test_polynomial([4_i64, 2_i64, 0_i64].to_vec(), -4_i64, -4_i64);

        // f(x) = 3x^2 + 2x + 4
        test_polynomial([4_i64, 2_i64, 3_i64].to_vec(), 0_i64, 4_i64);
        test_polynomial([4_i64, 2_i64, 3_i64].to_vec(), 4_i64, 60_i64); // 48 + 8 + 4
        test_polynomial([4_i64, 2_i64, 3_i64].to_vec(), -4_i64, 44_i64); // 48 - 8 + 4
    }

    fn test_polynomial(coeffs: Vec<i64>, value: i64, expected_result: i64) {
        let pol: Polynomial = Polynomial::new(coeffs);
        assert_eq!(pol.evaluate(value), expected_result);
    }
}
