use super::polynomial::Polynomial;
use crate::finite_field::FiniteFieldElement;

impl Polynomial {
    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let a_len = self.coefficients.len();
        let b_len = other.coefficients.len();
        let max_len = if a_len > b_len { a_len } else { b_len };

        let mut result_coeffs: Vec<FiniteFieldElement> = vec![FiniteFieldElement::ZERO; max_len];

        // Copy the original
        for i in 0..self.coefficients.len() {
            result_coeffs[i] = self.coefficients[i];
        }

        // Subtract other in the field
        for i in 0..other.coefficients.len() {
            result_coeffs[i] = result_coeffs[i].subtract(other.coefficients[i]);
        }

        Polynomial::new_ff(result_coeffs).trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_empty() {
        // f(x) = 0
        let empty_poly = Polynomial::new([].to_vec());
        let non_empty_poly = Polynomial::new([5].to_vec());

        let res = empty_poly.sub(&empty_poly);
        assert_eq!(res.coefficients.len(), 0);

        let res = non_empty_poly.sub(&empty_poly);
        assert_eq!(res.coefficients.len(), 1);
        assert_eq!(res.coefficients[0].value, 5);

        let res = empty_poly.sub(&non_empty_poly);
        assert_eq!(res.coefficients.len(), 1);
        // 0 - 5 mod p == p - 5
        let p = FiniteFieldElement::DEFAULT_FIELD_SIZE;
        assert_eq!(res.coefficients[0].value, (p - 5) % p);
    }

    #[test]
    fn sub() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7, 2].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let res = poly1.sub(&poly2);

        assert_eq!(res.coefficients.len(), 3);
        assert_eq!(res.coefficients[0].value, 4);
        // 0 - 7 mod p == p - 7
        let p = FiniteFieldElement::DEFAULT_FIELD_SIZE;
        assert_eq!(res.coefficients[1].value, (p - 7) % p);
        assert_eq!(res.coefficients[2].value, 1);
    }
}
