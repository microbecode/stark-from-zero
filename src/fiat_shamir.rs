use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::hashing;

/// Minimal Fiat–Shamir transcript for educational purposes.
///
/// Accumulates data via a simple rolling hash and derives challenges
/// deterministically as field elements.
#[derive(Debug, Clone, Copy)]
pub struct Transcript {
    /// Internal sponge-like state (single word, educational)
    state: i128,
}

impl Transcript {
    pub fn new() -> Self {
        Transcript { state: 0 }
    }

    /// Absorb a single integer into the transcript
    pub fn absorb_i128(&mut self, value: i128) {
        // Simple rolling hash: hash(state || value)
        let mixed = self.state.wrapping_add(value.rotate_left(1));
        self.state = hashing::hash(mixed);
    }

    /// Absorb a byte slice by chunking into i128 limbs
    pub fn absorb_bytes(&mut self, bytes: &[u8]) {
        let mut acc: i128 = 0;
        let mut count: usize = 0;
        for &b in bytes {
            acc = (acc << 8) | (b as i128);
            count += 1;
            if count == 15 {
                // pack up to 15 bytes before absorbing
                self.absorb_i128(acc);
                acc = 0;
                count = 0;
            }
        }
        if count > 0 {
            self.absorb_i128(acc);
        }
    }

    /// Derive a challenge as a field element in the provided field
    pub fn challenge(&mut self, field: FiniteField) -> FiniteFieldElement {
        // domain separate by hashing state again
        self.state = hashing::hash(self.state.wrapping_add(0x9e37_79b9_7f4a_7c15));
        // Map hash to field by reduction
        FiniteFieldElement::new_fielded(self.state, field)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_FIELD_SIZE;

    use super::*;

    #[test]
    fn determinism_same_inputs_same_challenges() {
        let field = FiniteField::new(DEFAULT_FIELD_SIZE);
        let mut t1 = Transcript::new();
        let mut t2 = Transcript::new();

        t1.absorb_i128(42);
        t1.absorb_bytes(b"hello");
        let c1a = t1.challenge(field);
        let c1b = t1.challenge(field);

        t2.absorb_i128(42);
        t2.absorb_bytes(b"hello");
        let c2a = t2.challenge(field);
        let c2b = t2.challenge(field);

        assert_eq!(c1a.value, c2a.value);
        assert_eq!(c1b.value, c2b.value);
        assert_eq!(c1a.field.prime, field.prime);
        assert_eq!(c1b.field.prime, field.prime);
    }

    #[test]
    fn different_absorbs_change_challenge() {
        let field = FiniteField::new(DEFAULT_FIELD_SIZE);
        let mut t1 = Transcript::new();
        let mut t2 = Transcript::new();

        t1.absorb_i128(1);
        let c1 = t1.challenge(field);

        t2.absorb_i128(2);
        let c2 = t2.challenge(field);

        assert_ne!(c1.value, c2.value);
    }
}
