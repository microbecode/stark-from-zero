use super::polynomial::Polynomial;

impl Polynomial {
    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs =
            vec![0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

        // Copy the original
        for i in 0..self.coefficients.len() {
            result_coeffs[i] += self.coefficients[i];
        }

        for i in 0..other.coefficients.len() {
            result_coeffs[i] -= other.coefficients[i];
        }

        Polynomial {
            coefficients: result_coeffs,
        }
        .trim()
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
        assert_eq!(res.coefficients[0], 5);

        let res = empty_poly.sub(&non_empty_poly);
        assert_eq!(res.coefficients.len(), 1);
        assert_eq!(res.coefficients[0], -5);
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
        assert_eq!(res.coefficients[0], 4);
        assert_eq!(res.coefficients[1], -7);
        assert_eq!(res.coefficients[2], 1);
    }
}
