pub fn hash(input: i128) -> i128 {
    let mut hash: i128 = 3;
    let mut num = input.wrapping_mul(100003); // biggish prime to make all inputs of at least certain size
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
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn small_are_not_identical() {
        let mut found_hashes = HashMap::<i128, bool>::new();
        for i in 0..10000 {
            let hash = hash(i);
            assert!(!found_hashes.contains_key(&hash));
            found_hashes.insert(hash, true);
            // println!("hash {} {}", i, hash);
        }
    }

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

        let max_value: i128 = i128::MAX;

        // Test wrapping
        assert_ne!(hash(max_value), hash(max_value - 1));
        assert_ne!(hash(max_value), hash(max_value / 10));

        let max_value_str = max_value.to_string();
        let without_left_most_digit = &max_value_str[1..]; // Slice to remove the first character
        let without_left_most_digit = without_left_most_digit.parse::<i128>().unwrap();

        assert_ne!(hash(max_value), hash(without_left_most_digit));
    }
}
