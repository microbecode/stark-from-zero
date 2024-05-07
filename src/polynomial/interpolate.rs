use super::polynomial::Polynomial;

/// Adjusted from https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Polynomial_interpolation
pub fn lagrange_interpolation(points: &[(i128, i128)]) -> Polynomial {
    let numpts = points.len();

    let mut thepoly = vec![0.0; numpts];
    let mut theterm = vec![0.0; numpts];

    for i in 0..numpts {
        let mut prod = 1.0;
        for j in 0..numpts {
            theterm[j] = 0.0;
        }
        for j in 0..numpts {
            if i != j {
                prod *= (points[i].0 - points[j].0) as f64;
            }
        }

        theterm[0] = points[i].1 as f64 / prod;

        for j in 0..numpts {
            if i != j {
                for k in (1..numpts).rev() {
                    theterm[k] += theterm[k - 1];
                    theterm[k - 1] *= -points[j].0 as f64;
                }
            }
        }

        for j in 0..numpts {
            thepoly[j] += theterm[j];
        }
    }

    // Mutate to integers
    let mut result = Polynomial::new(vec![]);
    for i in thepoly {
        result.coefficients.push(i as i128);
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
}
