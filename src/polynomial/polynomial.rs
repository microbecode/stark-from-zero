use crate::finite_field::FiniteFieldElement;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<i128>,
}

impl Polynomial {
    /// Note that the coefficients are enumerated from the "right": the first entry is for constant value.
    /// The second entry is for coefficient * x, third entry for coefficient * x^2, etc
    pub fn new(coefficients: Vec<i128>) -> Self {
        Polynomial { coefficients }
    }

    /// Returns the degree of the polynomial
    pub fn degree(&self) -> usize {
        for i in (0..self.coefficients.len()).rev() {
            if self.coefficients[i] != self::FiniteFieldElement::ZERO.value {
                return i;
            }
        }
        return 0;
    }

    /// Returns a polynomial where all coefficients are zero except the highest term
    pub fn leading_term(&self) -> Polynomial {
        let highest_degree_index = self.coefficients.len() - 1;

        // Create a new vector to store the coefficients of the highest degree term
        let mut highest_degree_coefficients = vec![0_i128; highest_degree_index];
        highest_degree_coefficients.push(self.coefficients[highest_degree_index]);

        // Create a new polynomial with the highest degree term
        Polynomial {
            coefficients: highest_degree_coefficients,
        }
    }

    /// Remove all zero coefficients from the start
    pub fn trim(&self) -> Polynomial {
        // Find the index of the last non-zero coefficient from the end
        let end_index = self
            .coefficients
            .iter()
            .rposition(|&c| c != 0)
            .map_or(0, |i| i + 1);

        // Create a new polynomial with trimmed coefficients
        Polynomial {
            coefficients: self.coefficients[..end_index].to_vec(),
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = Vec::new();
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            if coeff != 0 {
                let degree = i;
                let term_str = match degree {
                    0 => format!("{}", coeff),
                    1 => format!("{}x", coeff),
                    _ => format!("{}x^{}", coeff, degree),
                };
                terms.push(term_str);
            }
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            terms.reverse();
            write!(f, "{}", terms.join(" + "))
        }
    }
}

/// Based on https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Polynomial_interpolation

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 0);

        let coeffs = [4_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 2);
    }

    #[test]
    fn leading_term() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        //  assert_eq!(poly1.leading_term().coefficients.len(), 1);
        assert_eq!(poly1.leading_term().coefficients[0], 4);

        let coeffs = [2_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0], 0);
        assert_eq!(poly1.leading_term().coefficients[2], 3);
    }

    #[test]
    fn trim() {
        let coeffs = vec![0_i128, 2, 4];
        let mut poly = Polynomial::new(coeffs.clone());
        poly = poly.trim();
        assert_vectors_equal(&poly.coefficients, &coeffs);

        let coeffs = vec![0_i128, 2, 0, 4, 0];
        let should_result = vec![0_i128, 2, 0, 4];
        let mut poly = Polynomial::new(coeffs.clone());
        poly = poly.trim();
        assert_vectors_equal(&poly.coefficients, &should_result);

        fn assert_vectors_equal(a: &[i128], b: &[i128]) {
            assert_eq!(a.len(), b.len()); // Ensure vectors have the same length

            // Compare each element of the vectors
            for (x, y) in a.iter().zip(b.iter()) {
                assert_eq!(x, y);
            }
        }
    }
}
