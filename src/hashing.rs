pub fn hash(input: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in input.chars() {
        let char_num = c as u32;
        hash = hash.wrapping_mul(100); // Shift
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

        assert_ne!(hash("ab"), hash("ba"));
        assert_ne!(hash("ab "), hash("ab"));
        assert_ne!(hash(" ab"), hash("ab"));
    }
}
