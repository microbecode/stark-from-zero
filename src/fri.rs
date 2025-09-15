use crate::finite_field::{FiniteField, FiniteFieldElement};

/// Minimal FRI-style folding over evaluations on a coset of size 2^k.
///
/// Given a vector of evaluations `values` of length N (N must be even), and a
/// folding challenge `beta`, returns a new vector of length N/2 defined by:
///    new[i] = values[i] + beta * values[i + N/2]
/// This is a common educational simplification of the FRI folding step.
pub fn fold_once(
    values: &[FiniteFieldElement],
    beta: FiniteFieldElement,
) -> Vec<FiniteFieldElement> {
    assert!(!values.is_empty(), "values must not be empty");
    assert!(values.len() % 2 == 0, "values length must be even");

    let half = values.len() / 2;
    let mut out = Vec::with_capacity(half);
    for i in 0..half {
        let a = values[i];
        let b = values[i + half];
        out.push(a.add(beta.multiply(b)));
    }
    out
}

/// Repeatedly folds until length <= target_len (power of two recommended).
/// Panics if target_len is 0 or not a divisor of the initial length by a power of two.
pub fn fold_until(
    values: &[FiniteFieldElement],
    betas: &[FiniteFieldElement],
    target_len: usize,
) -> Vec<FiniteFieldElement> {
    assert!(target_len > 0, "target_len must be > 0");
    assert!(
        values.len() >= target_len,
        "target_len must be <= initial length"
    );
    assert!(
        values.len() % target_len == 0,
        "target_len must divide initial length"
    );

    let mut cur = values.to_vec();
    let mut beta_iter = betas.iter();
    while cur.len() > target_len {
        let beta = *beta_iter
            .next()
            .expect("not enough betas to fold to target_len");
        cur = fold_once(&cur, beta);
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_once_halves_length_and_is_deterministic() {
        let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
        // Build 8 sample values: 0..7
        let values: Vec<FiniteFieldElement> = (0..8)
            .map(|i| FiniteFieldElement::new_fielded(i as i128, field))
            .collect();
        let beta = FiniteFieldElement::new_fielded(3, field);

        let folded = fold_once(&values, beta);
        assert_eq!(folded.len(), 4);

        // Manual expectation: out[i] = v[i] + beta * v[i+4]
        for i in 0..4 {
            let expected = values[i].add(beta.multiply(values[i + 4]));
            assert_eq!(folded[i].value, expected.value);
            assert_eq!(folded[i].field.prime, expected.field.prime);
        }
    }

    #[test]
    fn fold_until_reduces_to_target_len() {
        let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
        let values: Vec<FiniteFieldElement> = (0..16)
            .map(|i| FiniteFieldElement::new_fielded((i as i128) * 7 + 1, field))
            .collect();

        // Two rounds needed from 16 -> 4
        let betas = vec![
            FiniteFieldElement::new_fielded(5, field),
            FiniteFieldElement::new_fielded(9, field),
        ];

        let out = fold_until(&values, &betas, 4);
        assert_eq!(out.len(), 4);

        // Check consistency with two fold_once rounds
        let after_one = fold_once(&values, betas[0]);
        let after_two = fold_once(&after_one, betas[1]);
        for (a, b) in out.iter().zip(after_two.iter()) {
            assert_eq!(a.value, b.value);
            assert_eq!(a.field.prime, b.field.prime);
        }
    }

    #[test]
    #[should_panic]
    fn fold_once_panics_on_odd_length() {
        let field = FiniteField::new(FiniteFieldElement::DEFAULT_FIELD_SIZE);
        let values: Vec<FiniteFieldElement> = (0..5)
            .map(|i| FiniteFieldElement::new_fielded(i as i128, field))
            .collect();
        let beta = FiniteFieldElement::new_fielded(2, field);
        let _ = fold_once(&values, beta);
    }
}
