use super::polynomial::Polynomial;

impl Polynomial {
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs =
            vec![0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

        // Copy original
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_empty() {
        // f(x) = 0
        let empty_poly = Polynomial::new([].to_vec());
        let non_empty_poly = Polynomial::new([5].to_vec());

        let added = empty_poly.add(&empty_poly);
        assert_eq!(added.coefficients.len(), 0);

        let added = non_empty_poly.add(&empty_poly);
        assert_eq!(added.coefficients.len(), 1);
        assert_eq!(added.coefficients[0], 5);

        let added = empty_poly.add(&non_empty_poly);
        assert_eq!(added.coefficients.len(), 1);
        assert_eq!(added.coefficients[0], 5);
    }

    #[test]
    fn add() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7, 2].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let added = poly1.add(&poly2);

        assert_eq!(added.coefficients.len(), 3);
        assert_eq!(added.coefficients[0], 4);
        assert_eq!(added.coefficients[1], 7);
        assert_eq!(added.coefficients[2], 5);
    }
}
