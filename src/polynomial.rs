use core::fmt;

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<i128>,
}

impl Polynomial {
    /// Note that the coefficients are enumerated from the "right": the first entry is for constant value.
    /// The second entry is for coefficient * x, third entry for coefficient * x^2, etc
    pub fn new(coefficients: Vec<i128>) -> Self {
        Polynomial { coefficients }
    }

    pub fn evaluate(&self, x: i128) -> i128 {
        let mut result = 0_i128;
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result += coeff * x.pow(i as u32);
        }
        result
    }

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

    pub fn div_scalar(&self, scalar: i128) -> Polynomial {
        let mut result_coeffs = self.coefficients.clone();
        for coeff in &mut result_coeffs {
            *coeff /= scalar;
        }
        Polynomial {
            coefficients: result_coeffs,
        }
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs =
            vec![0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

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
            vec![0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];

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
pub fn lagrange_interpolation(points: &[(i128, i128)]) -> Polynomial {
    let numpts = points.len();
    let mut theterm = vec![];
    let mut thepoly = vec![];

    for _ in 0..numpts {
        thepoly.push(0.0);
        theterm.push(0.0);
    }
    for i in 0..numpts {
        let mut prod = 1.0;
        for j in 0..numpts {
            theterm[j] = 0.0;
        }
        for j in 0..numpts {
            if i == j {
                continue;
            }
            prod *= points[i].0 as f64 - points[j].0 as f64;
        }

        prod = points[i].1 as f64 / prod;
        theterm[0] = prod;

        for j in 0..numpts {
            if i == j {
                continue;
            }
            for k in (1..numpts).rev() {
                theterm[k] += theterm[k - 1];
                theterm[k - 1] *= -points[j].0 as f64;
            }
        }

        for j in 0..numpts {
            thepoly[j] += theterm[j];
        }
    }

    // Mutate to integers
    let mut result = Polynomial::new(vec![]);
    for i in 0..thepoly.len() {
        result.coefficients.push(thepoly[i] as i128);
    }
    result
}

#[cfg(test)]
mod tests {
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

    fn test_polynomial_eval(coeffs: Vec<i128>, value: i128, expected_result: i128) {
        let pol: Polynomial = Polynomial::new(coeffs);
        assert_eq!(pol.evaluate(value), expected_result);
    }

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
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7_i128, 2_i128].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let added = poly1.add(&poly2);

        assert_eq!(added.coefficients.len(), 3);
        assert_eq!(added.coefficients[0], 4);
        assert_eq!(added.coefficients[1], 7);
        assert_eq!(added.coefficients[2], 5);
    }

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
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7_i128, 2_i128].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let res = poly1.sub(&poly2);

        assert_eq!(res.coefficients.len(), 3);
        assert_eq!(res.coefficients[0], 4);
        assert_eq!(res.coefficients[1], -7);
        assert_eq!(res.coefficients[2], 1);
    }

    #[test]
    fn degree() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 0);

        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.degree(), 2);
    }

    #[test]
    fn leading_term() {
        let coeffs = [4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients.len(), 1);
        assert_eq!(poly1.leading_term().coefficients[0], 4);

        let coeffs = [2_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0], 3);
    }

    #[test]
    fn lagrange_no_points() {
        let points = vec![];

        let poly = lagrange_interpolation(&points);

        assert_eq!(poly.coefficients.len(), 0);
    }

    #[test]
    fn lagrange_one_point() {
        let points = vec![(1, 2)];

        let poly = lagrange_interpolation(&points);

        // f(x) = 2
        assert_eq!(poly.coefficients.len(), 1);
        assert_eq!(poly.coefficients[0], 2);
    }

    #[test]
    fn lagrange_two_points() {
        let points = vec![(1, 2), (2, 4)];

        let poly = lagrange_interpolation(&points);
        // -4x + 12
        assert_eq!(poly.coefficients.len(), 2);
        assert_eq!(poly.coefficients[0], 0);
        assert_eq!(poly.coefficients[1], 2);

        // Same result if points are other way around
        let points = vec![(2, 4), (1, 2)];

        let poly = lagrange_interpolation(&points);
        // -4x + 12
        assert_eq!(poly.coefficients.len(), 2);
        assert_eq!(poly.coefficients[0], 0);
        assert_eq!(poly.coefficients[1], 2);
    }

    #[test]
    fn lagrange_three_points() {
        let points = vec![(0, -2), (1, 6), (-5, 48)];

        let poly = lagrange_interpolation(&points);
        // https://www.wolframalpha.com/input?i=interpolating+polynomial+calculator&assumption=%7B%22F%22%2C+%22InterpolatingPolynomialCalculator%22%2C+%22data2%22%7D+-%3E%22%7B%280%2C+-2%29%2C+%281%2C+6%29%2C+%28-5%2C+48%29%7D%22
        // f(x) = 3x^2  + 5x - 2
        println!("POL, {}", poly);
        assert_eq!(poly.coefficients.len(), 3);
        assert_eq!(poly.coefficients[0], -2);
        assert_eq!(poly.coefficients[1], 5);
        assert_eq!(poly.coefficients[2], 3);
    }

    #[test]
    fn lagrange_three_points2() {
        let points = vec![(1, 1), (2, 4), (3, 9)];

        let poly = lagrange_interpolation(&points);
        // f(x) = x^2
        assert_eq!(poly.coefficients.len(), 3);
        assert_eq!(poly.coefficients[0], 0);
        assert_eq!(poly.coefficients[1], 0);
        assert_eq!(poly.coefficients[2], 1);
    }

    pub fn div(a: &Polynomial, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        // Ensure that the divisor is not zero
        if divisor.coefficients.iter().all(|&c| c == 0) {
            panic!("Division by zero");
        }

        // Initialize quotient and remainder

        let mut quotient = Polynomial {
            coefficients: vec![0],
        };
        let mut remainder = a.clone();

        let mut i = 0;

        // Iterate until the degree of the remainder is less than the degree of the divisor
        while remainder.coefficients.len() > 0
            && remainder.coefficients[0] > 0
            && remainder.degree() > divisor.degree()
        {
            i += 1;
            if i > 5 {
                println!("Halted");
                break;
            }
            let t: i128 =
                remainder.leading_term().coefficients[0] / divisor.leading_term().coefficients[0];

            let cos = vec![t];
            let t_poly: Polynomial = Polynomial { coefficients: cos };

            quotient = quotient.add(&t_poly);
            remainder = remainder.sub(&t_poly).multiply(divisor);
        }

        (quotient, remainder)
    }

    /*   #[test]
    fn div_empty() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0_i128, 3_i128].to_vec();
        let non_empty = Polynomial::new(coeffs);

        let coeffs = [0_i128].to_vec();
        let empty = Polynomial::new(coeffs);

        let (q, r) = empty.div(&non_empty);
        assert_eq!(q.degree(), 0);
        assert_eq!(q.coefficients[0], 0.0);
        assert_eq!(r.degree(), 0);
        assert_eq!(r.coefficients[0], 0.0);
    }

    */

    #[test]
    fn div_simple() {
        // f(x) = 4x^2 + 2x
        let coeffs = [0_i128, 2_i128, 4_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);

        // x + 0
        let coeffs = [0_i128, 1_i128].to_vec();
        let poly2 = Polynomial::new(coeffs);

        let (q, r) = div(&poly1, &poly2);

        println!("PPOLY {}, {}", q, r);

        // 4x + 2
        /*         assert_eq!(q.coefficients.len(), 2);
        assert_eq!(q.coefficients[0], 2.0);
        assert_eq!(q.coefficients[1], 4.0);

        assert_eq!(r.coefficients.len(), 1);
        assert_eq!(r.coefficients[0], 0.0); */
    }
}
