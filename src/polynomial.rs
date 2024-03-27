pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    /// Note that the coefficients are enumerated from the "right": the first entry is for constant value.
    /// The second entry is for coefficient * x, third entry for coefficient * x^2, etc
    pub fn new(coefficients: Vec<f64>) -> Self {
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0_f64;
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result += coeff * x.powf(i as f64);
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
        test_polynomial([].to_vec(), 0_f64, 0_f64);
        test_polynomial([].to_vec(), 5_f64, 0_f64);
        test_polynomial([].to_vec(), -5_f64, 0_f64);
    }

    #[test]
    fn evaluation_single() {
        // f(X) = a
        test_polynomial([0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial([0_f64].to_vec(), 5_f64, 0_f64);
        test_polynomial([0_f64].to_vec(), 5_f64, 0_f64);

        test_polynomial([1_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial([1_f64].to_vec(), 1_f64, 1_f64);
        test_polynomial([1_f64].to_vec(), 999_f64, 1_f64);
        test_polynomial([1_f64].to_vec(), -999_f64, 1_f64);

        test_polynomial([4_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial([4_f64].to_vec(), 1_f64, 4_f64);
        test_polynomial([4_f64].to_vec(), 999_f64, 4_f64);
    }

    #[test]
    fn evaluation_double() {
        // f(x) = ax + b

        // f(x) = 0x + 0
        test_polynomial([0_f64, 0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial([0_f64, 0_f64].to_vec(), 4_f64, 0_f64);
        test_polynomial([0_f64, 0_f64].to_vec(), -4_f64, 0_f64);

        // f(x) = 0x + 1
        test_polynomial([1_f64, 0_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial([1_f64, 0_f64].to_vec(), 4_f64, 1_f64);
        test_polynomial([1_f64, 0_f64].to_vec(), -4_f64, 1_f64);

        // f(x) = 2x + 4
        test_polynomial([4_f64, 2_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial([4_f64, 2_f64].to_vec(), 4_f64, 12_f64);
        test_polynomial([4_f64, 2_f64].to_vec(), -4_f64, -4_f64);
    }

    #[test]
    fn evaluation_double_floating() {
        // f(x) = ax + b

        // f(x) = 2x + 4
        test_polynomial([4_f64, 2_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial([4_f64, 2_f64].to_vec(), 4.5_f64, 13_f64);
        test_polynomial([4_f64, 2_f64].to_vec(), -4.5_f64, -5_f64);
    }

    #[test]
    fn evaluation_triple() {
        // f(x) = ax^2 + bx + c

        // f(x) = 0x^2 + 0x + 0
        test_polynomial([0_f64, 0_f64, 0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial([0_f64, 0_f64, 0_f64].to_vec(), 4_f64, 0_f64);
        test_polynomial([0_f64, 0_f64, 0_f64].to_vec(), -4_f64, 0_f64);

        // f(x) = 0x^2 + 0x + 1
        test_polynomial([1_f64, 0_f64, 0_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial([1_f64, 0_f64, 0_f64].to_vec(), 4_f64, 1_f64);
        test_polynomial([1_f64, 0_f64, 0_f64].to_vec(), -4_f64, 1_f64);

        // f(x) = 0x^2 + 2x + 4
        test_polynomial([4_f64, 2_f64, 0_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial([4_f64, 2_f64, 0_f64].to_vec(), 4_f64, 12_f64);
        test_polynomial([4_f64, 2_f64, 0_f64].to_vec(), -4_f64, -4_f64);

        // f(x) = 3x^2 + 2x + 4
        test_polynomial([4_f64, 2_f64, 3_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial([4_f64, 2_f64, 3_f64].to_vec(), 4_f64, 60_f64); // 48 + 8 + 4
        test_polynomial([4_f64, 2_f64, 3_f64].to_vec(), -4_f64, 44_f64); // 48 - 8 + 4
    }

    fn test_polynomial(coeffs: Vec<f64>, value: f64, expected_result: f64) {
        let pol: Polynomial = Polynomial::new(coeffs);
        assert_eq!(pol.evaluate(value), expected_result);
    }
}
