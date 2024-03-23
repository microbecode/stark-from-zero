pub fn hash(input: &str) -> u64 {
    let mut hash: u64 = 0;
    for c in input.chars() {
        let char_num = c as u64;
        hash = hash.wrapping_mul(113); // Shift by a prime number so char positions make a difference
        hash = hash.wrapping_add(char_num);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        assert!(hash("hello") > 0);
        assert!(hash("world") > 0);
        assert_eq!(hash("hello"), hash("hello"));
        assert_ne!(hash("hello"), hash("world"));

        assert_ne!(hash("hello"), hash("ehllo"));
        assert_ne!(hash("hello"), hash("olleh"));

        assert_ne!(hash("ab"), hash("ba"));
        assert_ne!(hash("ab "), hash("ab"));
        assert_ne!(hash(" ab"), hash("ab"));

        // Test wrapping
        assert_ne!(
            hash("abc1 abc2 abc3 abc4 abc5 abc6 abc7 abc8 abc9"),
            hash("abc1 abc2 abc3 abc4 abc5 abc6 abc7 abc8 abc")
        );
        assert_ne!(
            hash("abc1 abc2 abc3 abc4 abc5 abc6"),
            hash("bc1 abc2 abc3 abc4 abc5 abc6")
        );
        // FIXME: test fails
    }
}
