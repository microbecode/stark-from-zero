use crate::number::modulo_multiply;

use super::polynomial::Polynomial;

impl Polynomial {
    pub fn div_scalar(&self, scalar: i128) -> Polynomial {
        let mut result_coeffs = self.coefficients.clone();
        for coeff in &mut result_coeffs {
            *coeff /= scalar;
        }
        Polynomial {
            coefficients: result_coeffs,
        }
    }

    pub fn div(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        // Ensure that the divisor is not zero
        if divisor.coefficients.iter().all(|&c| c == 0) {
            panic!("Division by zero");
        }

        let dividend_degree = self.degree();
        let divisor_degree = divisor.degree();

        if dividend_degree < divisor_degree {
            panic!("Invalid division");
        }

        let mut dividend = self.coefficients.to_vec();
        let mut quotient_coeffs = vec![0; dividend_degree - divisor_degree + 1];

        // Perform polynomial long division
        for i in (0..quotient_coeffs.len()).rev() {
            let current_degree = divisor_degree + i;
            if current_degree < dividend.len() && dividend[current_degree] != 0 {
                let quot = dividend[current_degree] / divisor.coefficients[divisor_degree];
                quotient_coeffs[i] = quot;

                // Subtract quot * divisor from dividend
                for j in 0..=divisor_degree {
                    if current_degree - divisor_degree + j < dividend.len() {
                        dividend[current_degree - divisor_degree + j] -=
                            divisor.coefficients[j] * quot;
                    }
                }
            }
        }

        let quotient = Polynomial::new(quotient_coeffs);
        let remainder = Polynomial::new(dividend).trim();

        (quotient, remainder)
    }

    pub fn div_modulo(&self, divisor: &Polynomial, modulus: i128) -> (Polynomial, Polynomial) {
        // Ensure that the divisor is not zero
        if divisor.coefficients.iter().all(|&c| c == 0) {
            panic!("Division by zero");
        }

        let mut apos = self.degree();
        let mut a = self.coefficients.to_vec();
        let bpos = divisor.degree();

        if apos < bpos {
            panic!("Invalid division");
        }

        let mut result = Polynomial {
            coefficients: vec![0; apos - bpos + 1],
        };

        for i in (0..result.coefficients.len()).rev() {
            let quot = a[apos] / divisor.coefficients[bpos];
            result.coefficients[i] = quot;
            for j in (0..bpos).rev() {
                //a[i + j] -= divisor.coefficients[j] * quot;
                a[i + j] -= modulo_multiply(divisor.coefficients[j], quot, modulus);
            }
            apos -= 1;
        }
        let remainder = self.sub(&result.multiply(divisor));

        (result, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_scalar() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        let multiplied = poly1.div_scalar(3);

        // (3x^2 + 0x + 4) / 3 = 1x^2 + 0x + 1
        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], (4 / 3));
        assert_eq!(multiplied.coefficients[1], 0);
        assert_eq!(multiplied.coefficients[2], 1);
    }

    #[test]
    #[should_panic(expected = "Invalid division")]
    fn div_empty() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let non_empty = Polynomial::new(coeffs);

        let coeffs = [0_i128].to_vec();
        let empty = Polynomial::new(coeffs);

        empty.div(&non_empty);
    }

    #[test]
    fn div_no_remainder() {
        // f(x) = 4x^2 + 2x
        let coeffs = [0_i128, 2_i128, 4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // x + 0
        let coeffs = [0_i128, 1_i128].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let (q, r) = poly1.div(&poly2);

        // 4x + 2
        assert_eq!(q.coefficients.len(), 2);
        assert_eq!(q.coefficients[0], 2);
        assert_eq!(q.coefficients[1], 4);

        assert_eq!(r.coefficients.len(), 0);
    }

    #[test]
    fn div_no_remainder2() {
        // f(x) = x^3 + x^2 + 2 * x + 2
        let coeffs = [2_i128, 2, 1, 1].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // x^2 + 2
        let coeffs = [2_i128, 0, 1].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let (q, r) = poly1.div(&poly2);

        // x + 1
        assert_eq!(q.coefficients.len(), 2);
        assert_eq!(q.coefficients[0], 1);
        assert_eq!(q.coefficients[1], 1);

        assert_eq!(r.coefficients.len(), 0);
    }

    #[test]
    fn div_remainder() {
        // f(x) = x^3 - 2x^2 - 4
        let coeffs = [-4_i128, 0, -2, 1].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // x - 3
        let coeffs = [-3_i128, 1].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let (q, r) = poly1.div(&poly2);

        // x^2 + x + 3 , remainder: 5
        assert_eq!(q.coefficients.len(), 3);
        assert_eq!(q.coefficients[0], 3);
        assert_eq!(q.coefficients[1], 1);
        assert_eq!(q.coefficients[2], 1);

        assert_eq!(r.coefficients.len(), 1);
        assert_eq!(r.coefficients[0], 5);
    }

    #[test]
    fn div_remainder2() {
        // f(x) = 6x^4 + 5x^3 + 4x - 4
        let coeffs = [-4_i128, 4, 0, 5, 6].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // 2x^2 + x - 1
        let coeffs = [-1_i128, 1, 2].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let (q, r) = poly1.div(&poly2);

        // 3x^2 + x + 1 , remainder: 4x - 3
        assert_eq!(q.coefficients.len(), 3);
        assert_eq!(q.coefficients[0], 1);
        assert_eq!(q.coefficients[1], 1);
        assert_eq!(q.coefficients[2], 3);

        assert_eq!(r.coefficients.len(), 2);
        assert_eq!(r.coefficients[0], -3);
        assert_eq!(r.coefficients[1], 4);
    }
}
