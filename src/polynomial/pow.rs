use super::polynomial::Polynomial;

impl Polynomial {
    // From ChatGPT
    pub fn pow(&self, other: i128) -> Self {
        if other == 0 {
            // If the exponent is 0, return the identity polynomial, which is the polynomial representing the constant term 1.
            Polynomial::new(vec![1])
        } else {
            let mut res = Polynomial::new(vec![1]);
            let mut current = self.clone();
            let mut exponent = other;
            while exponent > 0 {
                if exponent % 2 == 1 {
                    // If the current exponent is odd, multiply the result by the current polynomial.
                    res = res.multiply(&current.clone());
                }
                // Square the current polynomial.
                current = current.clone().multiply(&current);
                // Divide the exponent by 2 for the next iteration.
                exponent /= 2;
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow_zero() {
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly = Polynomial::new(coeffs);
        assert_eq!(poly.pow(0).coefficients.len(), 1);
        assert_eq!(poly.pow(0).coefficients[0], 1);

        let coeffs = [0_i128, 7, 2].to_vec();
        let poly = Polynomial::new(coeffs);
        assert_eq!(poly.pow(0).coefficients.len(), 1);
        assert_eq!(poly.pow(0).coefficients[0], 1);
    }

    #[test]
    fn pow_one() {
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(poly.pow(1).coefficients, &coeffs);

        let coeffs = [0_i128, 7, 2].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(poly.pow(1).coefficients, &coeffs);
    }

    #[test]
    fn pow_two() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(poly.pow(2).coefficients, &[16, 0, 24, 0, 9]);

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7, 2].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(poly.pow(2).coefficients, &[0, 0, 49, 28, 4]);
    }

    #[test]
    fn pow() {
        // f(x) = 3x^2 + 0x + 4
        let coeffs = [4_i128, 0, 3].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(
            poly.pow(10).coefficients,
            &[
                1048576, 0, 7864320, 0, 26542080, 0, 53084160, 0, 69672960, 0, 62705664, 0,
                39191040, 0, 16796160, 0, 4723920, 0, 787320, 0, 59049,
            ],
        );

        // f(x) = 2x^2 + 7x + 0
        let coeffs = [0_i128, 7, 2].to_vec();
        let poly = Polynomial::new(coeffs.clone());
        assert_vectors_equal(
            poly.pow(6).coefficients,
            &[
                0, 0, 0, 0, 0, 0, 117649, 201684, 144060, 54880, 11760, 1344, 64,
            ],
        );
    }

    fn assert_vectors_equal(a: Vec<i128>, b: &[i128]) {
        assert_eq!(a.len(), b.len()); // Ensure vectors have the same length

        // Compare each element of the vectors
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x, y);
        }
    }
}
