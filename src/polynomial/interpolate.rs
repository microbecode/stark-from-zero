use super::polynomial::Polynomial;
use crate::finite_field::FiniteFieldElement;

/// Lagrange interpolation over a finite field
///
/// Given points (x_i, y_i), returns the unique polynomial P(x)
/// such that P(x_i) = y_i for all i.
pub fn lagrange_interpolation(points: &[(i128, i128)]) -> Polynomial {
    let n = points.len();
    if n == 0 {
        return Polynomial::new(vec![]);
    }

    // Start with the zero polynomial
    let mut result = Polynomial::new(vec![]);

    // Classic Lagrange basis construction over a finite field:
    // P(x) = Σ_i y_i · L_i(x)
    // where L_i(x) = ∏_{j≠i} (x − x_j) / (x_i − x_j)
    for i in 0..n {
        let (xi, yi) = points[i];

        // Build numerator: ∏_{j≠i} (x − x_j)
        // Using our coeff convention, (x − x_j) is represented as [-x_j, 1]
        let mut basis = Polynomial::new(vec![1]); // 1 as a polynomial
        for j in 0..n {
            if i == j {
                continue;
            }
            let xj = points[j].0;
            basis = basis.multiply(&Polynomial::new(vec![-xj, 1]));
        }

        // Denominator: ∏_{j≠i} (x_i − x_j) in the field
        let mut denom = FiniteFieldElement::new(1);
        for j in 0..n {
            if i == j {
                continue;
            }
            let xj = points[j].0;
            denom = denom.multiply(FiniteFieldElement::new(xi - xj));
        }

        // Scale basis by y_i * denom^{-1} in the field, then accumulate
        let scale = FiniteFieldElement::new(yi).multiply(denom.inverse());
        let scaled = basis.multiply_scalar(scale.value);
        result = result.add(&scaled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // Representation: [2] (constant term only)
        assert_eq!(poly.coefficients.len(), 1);
        assert_eq!(poly.coefficients[0].value, 2);
    }

    #[test]
    fn lagrange_two_points() {
        // Points (1, 2), (2, 4) ⇒ line f(x) = 2x
        // Representation: [0, 2]
        let points = vec![(1, 2), (2, 4)];

        let poly = lagrange_interpolation(&points);
        // Expected: 2x (coeffs [0, 2])
        assert_eq!(poly.to_i128_coeffs(), [0, 2]);

        // Same result if points are other way around
        let points = vec![(2, 4), (1, 2)];
        let poly = lagrange_interpolation(&points);
        assert_eq!(poly.to_i128_coeffs(), [0, 2]);
    }

    #[test]
    fn lagrange_three_points() {
        // Points (0, -2), (1, 6), (-5, 48)
        // Expected polynomial: f(x) = 3x^2 + 5x − 2
        // Representation: [-2, 5, 3]
        let points = vec![(0, -2), (1, 6), (-5, 48)];

        let poly = lagrange_interpolation(&points);
        assert_eq!(poly.to_i128_coeffs(), [-2, 5, 3]);
    }

    #[test]
    fn lagrange_three_points2() {
        // Points (1, 1), (2, 4), (3, 9)
        // Expected polynomial: f(x) = x^2
        // Representation: [0, 0, 1]
        let points = vec![(1, 1), (2, 4), (3, 9)];

        let poly = lagrange_interpolation(&points);
        assert_eq!(poly.to_i128_coeffs(), [0, 0, 1]);
    }
}
