use crate::finite_field::FiniteFieldElement;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<FiniteFieldElement>,
}

impl Polynomial {
    /// Construct from raw i128 coefficients, mapping into the finite field
    pub fn new(coefficients: Vec<i128>) -> Self {
        Polynomial {
            coefficients: coefficients
                .into_iter()
                .map(FiniteFieldElement::new)
                .collect(),
        }
    }

    /// Construct from finite field elements
    pub fn new_ff(coefficients: Vec<FiniteFieldElement>) -> Self {
        Polynomial { coefficients }
    }

    /// Returns the degree of the polynomial (highest non-zero coefficient in the field)
    pub fn degree(&self) -> usize {
        for i in (0..self.coefficients.len()).rev() {
            if !self.coefficients[i].is_zero() {
                return i;
            }
        }
        0
    }

    /// Returns a polynomial where all coefficients are zero except the highest term
    pub fn leading_term(&self) -> Polynomial {
        let highest_degree_index = self.coefficients.len() - 1;

        // Create a new vector to store the coefficients of the highest degree term
        let mut highest_degree_coefficients =
            vec![FiniteFieldElement::new(0); highest_degree_index];
        highest_degree_coefficients.push(self.coefficients[highest_degree_index].clone());

        // Create a new polynomial with the highest degree term
        Polynomial {
            coefficients: highest_degree_coefficients,
        }
    }

    /// Remove all zero coefficients from the end
    pub fn trim(&self) -> Polynomial {
        // Find the index of the last non-zero coefficient from the end
        let end_index = self
            .coefficients
            .iter()
            .rposition(|c| !c.is_zero())
            .map_or(0, |i| i + 1);

        // Create a new polynomial with trimmed coefficients
        Polynomial {
            coefficients: self.coefficients[..end_index].to_vec(),
        }
    }

    /// Convenience: export coefficients as i128 values
    pub fn to_i128_coeffs(&self) -> Vec<i128> {
        self.coefficients.iter().map(|c| c.value).collect()
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = Vec::new();
        for (i, coeff) in self.coefficients.iter().enumerate() {
            if !coeff.is_zero() {
                let degree = i;
                let term_str = match degree {
                    0 => format!("{}", coeff.value),
                    1 => format!("{}x", coeff.value),
                    _ => format!("{}x^{}", coeff.value, degree),
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

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_FIELD_SIZE;

    use super::*;

    #[test]
    fn degree() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 0);

        let coeffs = [4_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 2);

        // Ensure negative multiples of field are treated as zero
        let p = DEFAULT_FIELD_SIZE;
        let coeffs = [1_i128, -(p), 0, 0].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 0);
    }

    #[test]
    fn leading_term() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0].value, 4);

        let coeffs = [2_i128, 0, 3].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0].value, 0);
        assert_eq!(poly1.leading_term().coefficients[2].value, 3);
    }

    #[test]
    fn trim() {
        let coeffs = vec![0_i128, 2, 4];
        let mut poly = Polynomial::new(coeffs.clone());
        poly = poly.trim();
        assert_eq!(poly.to_i128_coeffs(), coeffs);

        let coeffs = vec![0_i128, 2, 0, 4, 0];
        let should_result = vec![0_i128, 2, 0, 4];
        let mut poly = Polynomial::new(coeffs.clone());
        poly = poly.trim();
        assert_eq!(poly.to_i128_coeffs(), should_result);

        // Ensure that coefficients equal to 0 mod p are trimmed
        let p = DEFAULT_FIELD_SIZE;
        let coeffs = vec![0_i128, 2, -p, 0];
        let should_result = vec![0_i128, 2];
        let poly = Polynomial::new(coeffs);
        let poly = poly.trim();
        assert_eq!(poly.to_i128_coeffs(), should_result);
    }
}
