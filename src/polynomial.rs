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

    /// https://github.com/facebook/winterfell/blob/09525751727a283dfbf5e569a09909b82380e059/math/src/polynom/mod.rs#L397
    pub fn div(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
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
                a[i + j] -= divisor.coefficients[j] * quot;
            }
            apos = apos.wrapping_sub(1);
        }
        let remainder = self.sub(&result.multiply(divisor));

        (result, remainder)
    }

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

    /// Returns the degree of the polynomial
    pub fn degree(&self) -> usize {
        for i in (0..self.coefficients.len()).rev() {
            if self.coefficients[i] != 0 {
                return i;
            }
        }
        return 0;
    }

    /// Returns a polynomial where all coefficients are zero except the highest term
    pub fn leading_term(&self) -> Polynomial {
        let highest_degree_index = self.coefficients.len() - 1;

        // Create a new vector to store the coefficients of the highest degree term
        let mut highest_degree_coefficients = vec![0; highest_degree_index];
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
        //  assert_eq!(poly1.leading_term().coefficients.len(), 1);
        assert_eq!(poly1.leading_term().coefficients[0], 4);

        let coeffs = [2_i128, 0_i128, 3_i128].to_vec();
        let poly1 = Polynomial::new(coeffs);
        assert_eq!(poly1.leading_term().coefficients[0], 0);
        assert_eq!(poly1.leading_term().coefficients[2], 3);
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
    }

    fn assert_vectors_equal(a: &[i128], b: &[i128]) {
        assert_eq!(a.len(), b.len()); // Ensure vectors have the same length

        // Compare each element of the vectors
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x, y);
        }
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
        println!("RESULT {} , {}", q, r);
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
        println!("RESULT {} , {}", q, r);
        assert_eq!(q.coefficients.len(), 3);
        assert_eq!(q.coefficients[0], 1);
        assert_eq!(q.coefficients[1], 1);
        assert_eq!(q.coefficients[2], 3);

        assert_eq!(r.coefficients.len(), 2);
        assert_eq!(r.coefficients[0], -3);
        assert_eq!(r.coefficients[1], 4);
    }
}
