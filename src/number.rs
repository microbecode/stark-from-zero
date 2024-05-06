pub fn modulo_multiply(a: i128, b: i128, modulus: i128) -> i128 {
    let mut result = 0;
    let mut a = a % modulus; // Ensure a is within the modulus range
    let mut b = b % modulus; // Ensure b is within the modulus range

    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % modulus;
        }
        a = (a * 2) % modulus;
        b /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo_multiply_no_overflow() {
        assert_eq!(modulo_multiply(5, 4, 1000), 20);
        assert_eq!(modulo_multiply(5, 40, 1000), 200);
    }

    #[test]
    fn modulo_multiply_with_overflow() {
        // regular overflow
        assert_eq!(modulo_multiply(5, 4, 10), 0);
        assert_eq!(modulo_multiply(5, 4, 9), 2);

        // initial numbers overflow
        assert_eq!(modulo_multiply(14, 12, 10), 8);
        assert_eq!(modulo_multiply(14, 3, 10), 2);
        assert_eq!(modulo_multiply(3, 12, 10), 6);
    }
}
