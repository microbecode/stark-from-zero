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

    // Multiply the polynomial by a scalar
    pub fn multiply_scalar(&self, scalar: f64) -> Polynomial {
        Polynomial {
            coefficients: self
                .coefficients
                .iter()
                .map(|&coeff| coeff * scalar)
                .collect(),
        }
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, &coeff1) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result[i + j] += coeff1 * coeff2;
            }
        }

        Polynomial::new(result)
    }

    pub fn div_scalar(&self, scalar: f64) -> Polynomial {
        let mut result_coeffs = self.coefficients.clone();
        for coeff in &mut result_coeffs {
            *coeff /= scalar;
        }
        Polynomial {
            coefficients: result_coeffs,
        }
    }

    /*   pub fn divide(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        // Initialize quotient and remainder
        let mut quotient = Polynomial { coefficients: vec![0]; };
        let mut remainder = self.clone();

        // Loop until degree of remainder is less than degree of divisor
        while remainder.degree() >= divisor.degree() {
            // Find leading terms of remainder and divisor
            let lt_remainder = remainder.leading_term();
            let lt_divisor = divisor.leading_term();

            // Compute the next term of the quotient
            let term = lt_remainder / lt_divisor;

            // Update quotient and remainder
            quotient = quotient.add(&term);
            remainder = remainder.sub(&term.multiply(divisor));
        }

        (quotient, remainder)
    }  */

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs =
            vec![0.0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

        for i in 0..self.coefficients.len() {
            result_coeffs[i] += self.coefficients[i];
        }

        for i in 0..other.coefficients.len() {
            result_coeffs[i] += other.coefficients[i];
        }

        Polynomial {
            coefficients: result_coeffs,
        }
    }

    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs =
            vec![0.0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

        for i in 0..self.coefficients.len() {
            result_coeffs[i] += self.coefficients[i];
        }

        for i in 0..other.coefficients.len() {
            result_coeffs[i] -= other.coefficients[i]; // Subtract instead of add
        }

        Polynomial {
            coefficients: result_coeffs,
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn leading_term(&self) -> Polynomial {
        Polynomial {
            coefficients: vec![self.coefficients[self.degree()]],
        }
    }
}

pub fn lagrange_interpolation(points: &[(f64, f64)]) -> Polynomial {
    let mut interpolated_polynomial = Polynomial::new(vec![0.0]);

    for &(xi, yi) in points {
        let mut basis = Polynomial::new(vec![1.0]);
        for &(xj, _) in points.iter().filter(|&&(x, _)| x != xi) {
            basis = basis
                .multiply(&Polynomial::new(vec![-xj, 1.0]))
                .div_scalar(xi - xj);
        }
        interpolated_polynomial = interpolated_polynomial.add(&basis.multiply_scalar(yi));
    }

    interpolated_polynomial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation_none() {
        // f(X) = 0
        test_polynomial_eval([].to_vec(), 0_f64, 0_f64);
        test_polynomial_eval([].to_vec(), 5_f64, 0_f64);
        test_polynomial_eval([].to_vec(), -5_f64, 0_f64);
    }

    #[test]
    fn evaluation_single() {
        // f(X) = a
        test_polynomial_eval([0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial_eval([0_f64].to_vec(), 5_f64, 0_f64);
        test_polynomial_eval([0_f64].to_vec(), 5_f64, 0_f64);

        test_polynomial_eval([1_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial_eval([1_f64].to_vec(), 1_f64, 1_f64);
        test_polynomial_eval([1_f64].to_vec(), 999_f64, 1_f64);
        test_polynomial_eval([1_f64].to_vec(), -999_f64, 1_f64);

        test_polynomial_eval([4_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial_eval([4_f64].to_vec(), 1_f64, 4_f64);
        test_polynomial_eval([4_f64].to_vec(), 999_f64, 4_f64);
    }

    #[test]
    fn evaluation_double() {
        // f(x) = ax + b

        // f(x) = 0x + 0
        test_polynomial_eval([0_f64, 0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial_eval([0_f64, 0_f64].to_vec(), 4_f64, 0_f64);
        test_polynomial_eval([0_f64, 0_f64].to_vec(), -4_f64, 0_f64);

        // f(x) = 0x + 1
        test_polynomial_eval([1_f64, 0_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial_eval([1_f64, 0_f64].to_vec(), 4_f64, 1_f64);
        test_polynomial_eval([1_f64, 0_f64].to_vec(), -4_f64, 1_f64);

        // f(x) = 2x + 4
        test_polynomial_eval([4_f64, 2_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial_eval([4_f64, 2_f64].to_vec(), 4_f64, 12_f64);
        test_polynomial_eval([4_f64, 2_f64].to_vec(), -4_f64, -4_f64);
    }

    #[test]
    fn evaluation_double_floating() {
        // f(x) = ax + b

        // f(x) = 2x + 4
        test_polynomial_eval([4_f64, 2_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial_eval([4_f64, 2_f64].to_vec(), 4.5_f64, 13_f64);
        test_polynomial_eval([4_f64, 2_f64].to_vec(), -4.5_f64, -5_f64);
    }

    #[test]
    fn evaluation_triple() {
        // f(x) = ax^2 + bx + c

        // f(x) = 0x^2 + 0x + 0
        test_polynomial_eval([0_f64, 0_f64, 0_f64].to_vec(), 0_f64, 0_f64);
        test_polynomial_eval([0_f64, 0_f64, 0_f64].to_vec(), 4_f64, 0_f64);
        test_polynomial_eval([0_f64, 0_f64, 0_f64].to_vec(), -4_f64, 0_f64);

        // f(x) = 0x^2 + 0x + 1
        test_polynomial_eval([1_f64, 0_f64, 0_f64].to_vec(), 0_f64, 1_f64);
        test_polynomial_eval([1_f64, 0_f64, 0_f64].to_vec(), 4_f64, 1_f64);
        test_polynomial_eval([1_f64, 0_f64, 0_f64].to_vec(), -4_f64, 1_f64);

        // f(x) = 0x^2 + 2x + 4
        test_polynomial_eval([4_f64, 2_f64, 0_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial_eval([4_f64, 2_f64, 0_f64].to_vec(), 4_f64, 12_f64);
        test_polynomial_eval([4_f64, 2_f64, 0_f64].to_vec(), -4_f64, -4_f64);

        // f(x) = 3x^2 + 2x + 4
        test_polynomial_eval([4_f64, 2_f64, 3_f64].to_vec(), 0_f64, 4_f64);
        test_polynomial_eval([4_f64, 2_f64, 3_f64].to_vec(), 4_f64, 60_f64); // 48 + 8 + 4
        test_polynomial_eval([4_f64, 2_f64, 3_f64].to_vec(), -4_f64, 44_f64); // 48 - 8 + 4
    }

    fn test_polynomial_eval(coeffs: Vec<f64>, value: f64, expected_result: f64) {
        let pol: Polynomial = Polynomial::new(coeffs);
        assert_eq!(pol.evaluate(value), expected_result);
    }

    #[test]
    fn scalar_multiply_empty() {
        // f(x) = 0
        let coeffs = [].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(3.0);

        assert_eq!(multiplied.coefficients.len(), 0);
    }

    #[test]
    fn scalar_multiply_zero() {
        // f(x) = 3x^2 + 2x + 4
        let coeffs = [4_f64, 2_f64, 3_f64].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(0.0);

        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], 0.0);
        assert_eq!(multiplied.coefficients[1], 0.0);
        assert_eq!(multiplied.coefficients[2], 0.0);
    }

    #[test]
    fn scalar_multiply() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(3.0);

        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], 12.0);
        assert_eq!(multiplied.coefficients[1], 0.0);
        assert_eq!(multiplied.coefficients[2], 9.0);
    }

    #[test]
    fn multiply() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_f64, 7_f64, 2_f64].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let multiplied = poly1.multiply(&poly2);

        // (3x^2 + 4)(2x^2 + 7x) = 6x^4 + 21x^3 + 8x^2 + 28x + 0
        assert_eq!(multiplied.coefficients.len(), 5);
        assert_eq!(multiplied.coefficients[0], 0.0);
        assert_eq!(multiplied.coefficients[1], 28.0);
        assert_eq!(multiplied.coefficients[2], 8.0);
        assert_eq!(multiplied.coefficients[3], 21.0);
        assert_eq!(multiplied.coefficients[4], 6.0);
    }

    #[test]
    fn scalar_divide() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);

        let multiplied = poly1.div_scalar(3.0);

        // (3x^2 + 0x + 4) / 3 = 1x^2 + 0x + 1
        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], (4.0 / 3.0));
        assert_eq!(multiplied.coefficients[1], 0.0);
        assert_eq!(multiplied.coefficients[2], 1.0);
    }

    #[test]
    fn add_empty() {
        // f(x) = 0
        let empty_poly = Polynomial::new([].to_vec());
        let non_empty_poly = Polynomial::new([5.0].to_vec());

        let added = empty_poly.add(&empty_poly);
        assert_eq!(added.coefficients.len(), 0);

        let added = non_empty_poly.add(&empty_poly);
        assert_eq!(added.coefficients.len(), 1);
        assert_eq!(added.coefficients[0], 5.0);

        let added = empty_poly.add(&non_empty_poly);
        assert_eq!(added.coefficients.len(), 1);
        assert_eq!(added.coefficients[0], 5.0);
    }

    #[test]
    fn add() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_f64, 7_f64, 2_f64].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let added = poly1.add(&poly2);

        assert_eq!(added.coefficients.len(), 3);
        assert_eq!(added.coefficients[0], 4.0);
        assert_eq!(added.coefficients[1], 7.0);
        assert_eq!(added.coefficients[2], 5.0);
    }

    #[test]
    fn sub_empty() {
        // f(x) = 0
        let empty_poly = Polynomial::new([].to_vec());
        let non_empty_poly = Polynomial::new([5.0].to_vec());

        let res = empty_poly.sub(&empty_poly);
        assert_eq!(res.coefficients.len(), 0);

        let res = non_empty_poly.sub(&empty_poly);
        assert_eq!(res.coefficients.len(), 1);
        assert_eq!(res.coefficients[0], 5.0);

        let res = empty_poly.sub(&non_empty_poly);
        assert_eq!(res.coefficients.len(), 1);
        assert_eq!(res.coefficients[0], -5.0);
    }

    #[test]
    fn sub() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_f64, 7_f64, 2_f64].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let res = poly1.sub(&poly2);

        assert_eq!(res.coefficients.len(), 3);
        assert_eq!(res.coefficients[0], 4.0);
        assert_eq!(res.coefficients[1], -7.0);
        assert_eq!(res.coefficients[2], 1.0);
    }

    #[test]
    fn degree() {
        let coeffs = [4_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 0);

        let coeffs = [4_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 2);
    }

    #[test]
    fn leading_term() {
        let coeffs = [4_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients.len(), 1);
        assert_eq!(poly1.leading_term().coefficients[0], 4.0);

        let coeffs = [2_f64, 0_f64, 3_f64].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0], 3.0);
    }

    #[test]
    fn lagrange_no_points() {
        let points = vec![];

        let poly = lagrange_interpolation(&points);

        assert_eq!(poly.coefficients.len(), 1);
        assert_eq!(poly.coefficients[0], 0.0);
    }

    #[test]
    fn lagrange_one_point() {
        let points = vec![(1.0, 2.0)];

        let poly = lagrange_interpolation(&points);

        // f(x) = 2
        assert_eq!(poly.coefficients.len(), 1);
        assert_eq!(poly.coefficients[0], 2.0);
    }

    #[test]
    fn lagrange_two_points() {
        let points = vec![(1.0, 2.0), (2.0, 4.0)];

        let poly = lagrange_interpolation(&points);
        // -4x + 12
        assert_eq!(poly.coefficients.len(), 2);
        assert_eq!(poly.coefficients[0], 0.0);
        assert_eq!(poly.coefficients[1], 2.0);

        // Same result if points are other way around
        let points = vec![(2.0, 4.0), (1.0, 2.0)];

        let poly = lagrange_interpolation(&points);
        // -4x + 12
        assert_eq!(poly.coefficients.len(), 2);
        assert_eq!(poly.coefficients[0], 0.0);
        assert_eq!(poly.coefficients[1], 2.0);
    }

    #[test]
    fn lagrange_two_points_neg() {
        let points = vec![(-2.5, -1.0), (-1.0, -2.0)];

        let poly = lagrange_interpolation(&points);
        // have to round the numbers due to precision issues.
        // the exact polynomial is f(x) = -(8/6)x - (2/3)
        assert_eq!(poly.coefficients.len(), 2);
        assert_eq!(
            (poly.coefficients[0] * 1_000.0) as i64,
            (-8.0 / 3.0 * 1_000.0) as i64
        );
        assert_eq!(
            (poly.coefficients[1] * 1_000.0) as i64,
            (-2.0 / 3.0 * 1_000.0) as i64
        );
    }

    #[test]
    fn lagrange_three_points() {
        let points = vec![(0.0, -2.0), (1.0, 6.0), (-5.0, 48.0)];

        let poly = lagrange_interpolation(&points);
        // f(x) = 3x^2  + 5x - 2
        assert_eq!(poly.coefficients.len(), 3);
        assert_eq!(poly.coefficients[0], -2.0);
        assert_eq!(poly.coefficients[1], 5.0);
        assert_eq!(poly.coefficients[2], 3.0);
    }
}
