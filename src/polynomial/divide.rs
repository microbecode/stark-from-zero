use super::polynomial::Polynomial;
use crate::finite_field::FiniteFieldElement;

impl Polynomial {
    pub fn div_scalar(&self, scalar: i128) -> Polynomial {
        let scalar_elem = FiniteFieldElement::new(scalar);
        let inv = scalar_elem.inverse();
        let coeffs: Vec<FiniteFieldElement> =
            self.coefficients.iter().map(|c| c.multiply(inv)).collect();
        Polynomial::new_ff(coeffs)
    }

    pub fn div(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        // Ensure that the divisor is not zero
        if divisor.coefficients.iter().all(|c| c.is_zero()) {
            panic!("Division by zero");
        }

        let dividend_degree = self.degree();
        let divisor_degree = divisor.degree();

        if dividend_degree < divisor_degree {
            panic!("Invalid division");
        }

        // Working copy of dividend coefficients
        let mut dividend = self.coefficients.to_vec();
        let mut quotient_coeffs: Vec<FiniteFieldElement> =
            vec![FiniteFieldElement::ZERO; dividend_degree - divisor_degree + 1];

        // Leading coefficient of divisor and its inverse
        let lead_div = divisor.coefficients[divisor_degree];
        let lead_div_inv = lead_div.inverse();

        // Perform polynomial long division
        for i in (0..quotient_coeffs.len()).rev() {
            let current_degree = divisor_degree + i;
            if current_degree < dividend.len() && !dividend[current_degree].is_zero() {
                // quot = dividend_lead / divisor_lead
                let quot = dividend[current_degree].multiply(lead_div_inv);
                quotient_coeffs[i] = quot;

                // Subtract quot * divisor shifted
                for j in 0..=divisor_degree {
                    if current_degree - divisor_degree + j < dividend.len() {
                        let term = divisor.coefficients[j].multiply(quot);
                        let idx = current_degree - divisor_degree + j;
                        dividend[idx] = dividend[idx].subtract(term);
                    }
                }
            }
        }

        let quotient = Polynomial::new_ff(quotient_coeffs);
        let remainder = Polynomial::new_ff(dividend).trim();

        (quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_FIELD_SIZE;

    use super::*;

    #[test]
    fn div_scalar() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        let divided = poly1.div_scalar(3);

        // Check that multiplying back by 3 yields the original polynomial mod p
        let restored = divided.multiply_scalar(3);
        assert_eq!(restored.coefficients.len(), poly1.coefficients.len());
        for (a, b) in restored.coefficients.iter().zip(poly1.coefficients.iter()) {
            assert_eq!(a.value, b.value);
        }

        // Also check explicit values against inverse(3)
        let inv3 = FiniteFieldElement::new(3).inverse().value;
        let p = DEFAULT_FIELD_SIZE;
        assert_eq!(divided.coefficients[0].value, (4 * inv3) % p);
        assert_eq!(divided.coefficients[1].value, 0);
        assert_eq!(divided.coefficients[2].value, (3 * inv3) % p);
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
        assert_eq!(q.coefficients[0].value, 2);
        assert_eq!(q.coefficients[1].value, 4);

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
        assert_eq!(q.coefficients[0].value, 1);
        assert_eq!(q.coefficients[1].value, 1);

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
        assert_eq!(q.coefficients[0].value, 3);
        assert_eq!(q.coefficients[1].value, 1);
        assert_eq!(q.coefficients[2].value, 1);

        assert_eq!(r.coefficients.len(), 1);
        assert_eq!(r.coefficients[0].value, 5 % DEFAULT_FIELD_SIZE);
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
        assert_eq!(q.coefficients[0].value, 1);
        assert_eq!(q.coefficients[1].value, 1);
        assert_eq!(q.coefficients[2].value, 3);

        assert_eq!(r.coefficients.len(), 2);
        let p = DEFAULT_FIELD_SIZE;
        assert_eq!(r.coefficients[0].value, (p - 3) % p);
        assert_eq!(r.coefficients[1].value, 4);
    }
}
