use crate::finite_field::FiniteFieldElement;

use super::polynomial::Polynomial;

impl Polynomial {
    pub fn evaluate(&self, x: FiniteFieldElement) -> FiniteFieldElement {
        let mut result = FiniteFieldElement::new_fielded(0, x.field);
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            let co_elem = FiniteFieldElement::new_fielded(coeff, x.field);
            let pow = x.pow(i as i128);
            let multi = pow.multiply(co_elem);
            result = result.add(multi);
        }
        result
    }

    /// Adjusted from https://github.com/lambdaclass/STARK101-rs/blob/main/stark101/src/polynomial.rs#L264
    pub fn compose(&self, other: Polynomial) -> Polynomial {
        let mut res = Polynomial::new(vec![]);
        for coef in self.clone().coefficients.into_iter().rev() {
            res = other.multiply(&res).add(&Polynomial::new(vec![coef]));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_field::FiniteField;

    use super::*;

    #[test]
    fn evaluation_none() {
        // f(X) = 0
        test_polynomial_eval([].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([].to_vec(), 5_i128, 0_i128);
        test_polynomial_eval([].to_vec(), -5_i128, 0_i128);
    }

    #[test]
    fn evaluation_single() {
        // f(X) = a
        test_polynomial_eval([0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128].to_vec(), 5_i128, 0_i128);
        test_polynomial_eval([0_i128].to_vec(), 5_i128, 0_i128);

        test_polynomial_eval([1_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), 1_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), 999_i128, 1_i128);
        test_polynomial_eval([1_i128].to_vec(), -999_i128, 1_i128);

        test_polynomial_eval([4_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128].to_vec(), 1_i128, 4_i128);
        test_polynomial_eval([4_i128].to_vec(), 999_i128, 4_i128);
    }

    #[test]
    fn evaluation_double() {
        // f(x) = ax + b

        // f(x) = 0x + 0
        test_polynomial_eval([0_i128, 0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128].to_vec(), 4_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128].to_vec(), -4_i128, 0_i128);

        // f(x) = 0x + 1
        test_polynomial_eval([1_i128, 0_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128].to_vec(), 4_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128].to_vec(), -4_i128, 1_i128);

        // f(x) = 2x + 4
        test_polynomial_eval([4_i128, 2_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128].to_vec(), 4_i128, 12_i128);
        test_polynomial_eval([4_i128, 2_i128].to_vec(), -4_i128, -4_i128);
    }

    #[test]
    fn evaluation_triple() {
        // f(x) = ax^2 + bx + c

        // f(x) = 0x^2 + 0x + 0
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), 0_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), 4_i128, 0_i128);
        test_polynomial_eval([0_i128, 0_i128, 0_i128].to_vec(), -4_i128, 0_i128);

        // f(x) = 0x^2 + 0x + 1
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), 0_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), 4_i128, 1_i128);
        test_polynomial_eval([1_i128, 0_i128, 0_i128].to_vec(), -4_i128, 1_i128);

        // f(x) = 0x^2 + 2x + 4
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), 4_i128, 12_i128);
        test_polynomial_eval([4_i128, 2_i128, 0_i128].to_vec(), -4_i128, -4_i128);

        // f(x) = 3x^2 + 2x + 4
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), 0_i128, 4_i128);
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), 4_i128, 60_i128); // 48 + 8 + 4
        test_polynomial_eval([4_i128, 2_i128, 3_i128].to_vec(), -4_i128, 44_i128);
        // 48 - 8 + 4
    }

    #[test]
    fn evaluation_overflow() {
        let pol: Polynomial = Polynomial::new([0_i128, 0, 1].to_vec());
        let elem = FiniteFieldElement::new_fielded(4, FiniteField::new(10));
        assert_eq!(pol.evaluate(elem).value, 6);
    }

    fn test_polynomial_eval(coeffs: Vec<i128>, value: i128, expected_result: i128) {
        let pol: Polynomial = Polynomial::new(coeffs);
        let elem = FiniteFieldElement::new_fielded(value, FiniteField::new(i128::MAX));
        assert_eq!(pol.evaluate(elem).value, expected_result);
    }

    #[test]
    fn compose_trivial() {
        let first: Polynomial = Polynomial::new([0, 1].to_vec());
        let second: Polynomial = Polynomial::new([0, 1].to_vec());

        // x ∘ x
        assert_eq!(first.compose(second).coefficients, [0, 1]);
    }

    #[test]
    fn compose() {
        let first: Polynomial = Polynomial::new([0, 1, 1].to_vec());
        let second: Polynomial = Polynomial::new([1, 1].to_vec());

        // x^2 + x ∘ x + 1
        assert_eq!(first.compose(second).coefficients, [2, 3, 1]);
    }
}
