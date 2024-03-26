pub fn hash(input: u64) -> u64 {
    let mut hash: u64 = 0;
    let mut num = input;
    while num != 0 {
        let digit = num % 10;
        hash = hash.wrapping_mul(113); // Shift by a prime number so digit positions make a difference
        hash = hash.wrapping_add(digit);
        num /= 10;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        assert!(hash(2) > 0);
        assert!(hash(3) > 0);
        assert_eq!(hash(2), hash(2));
        assert_ne!(hash(2), hash(3));

        assert_ne!(hash(234), hash(324));
        assert_ne!(hash(234), hash(432));

        assert_ne!(hash(234), hash(23));
        assert_ne!(hash(234), hash(34));

        let max_value: u64 = u64::MAX;

        // Test wrapping
        assert_ne!(hash(max_value), hash(max_value - 1));
        assert_ne!(hash(max_value), hash(max_value / 10));

        let max_value_str = max_value.to_string();
        let without_left_most_digit = &max_value_str[1..]; // Slice to remove the first character
        let without_left_most_digit = without_left_most_digit.parse::<u64>().unwrap();

        assert_ne!(hash(max_value), hash(without_left_most_digit));
    }
}
