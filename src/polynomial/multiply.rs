use super::polynomial::Polynomial;

impl Polynomial {
    // Multiply the polynomial by a scalar
    pub fn multiply_scalar(&self, scalar: i128) -> Polynomial {
        Polynomial {
            coefficients: self
                .coefficients
                .iter()
                .map(|&coeff| coeff * scalar)
                .collect(),
        }
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, &coeff1) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result[i + j] += coeff1 * coeff2;
            }
        }

        Polynomial::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiply_empty() {
        // f(x) = 0
        let coeffs = [].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(3);

        assert_eq!(multiplied.coefficients.len(), 0);
    }

    #[test]
    fn scalar_multiply_zero() {
        // f(x) = 3x^2 + 2x + 4
        let coeffs = [4_i128, 2_i128, 3_i128].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(0);

        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], 0);
        assert_eq!(multiplied.coefficients[1], 0);
        assert_eq!(multiplied.coefficients[2], 0);
    }

    #[test]
    fn scalar_multiply() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly = Polynomial::new(coeffs);
        let multiplied = poly.multiply_scalar(3);

        assert_eq!(multiplied.coefficients.len(), 3);
        assert_eq!(multiplied.coefficients[0], 12);
        assert_eq!(multiplied.coefficients[1], 0);
        assert_eq!(multiplied.coefficients[2], 9);
    }

    #[test]
    fn multiply() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7_i128, 2_i128].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let multiplied = poly1.multiply(&poly2);

        // (3x^2 + 4)(2x^2 + 7x) = 6x^4 + 21x^3 + 8x^2 + 28x + 0
        assert_eq!(multiplied.coefficients.len(), 5);
        assert_eq!(multiplied.coefficients[0], 0);
        assert_eq!(multiplied.coefficients[1], 28);
        assert_eq!(multiplied.coefficients[2], 8);
        assert_eq!(multiplied.coefficients[3], 21);
        assert_eq!(multiplied.coefficients[4], 6);
    }
}
