use crate::hashing;

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    prime: u64,
}

impl FiniteField {
    fn new(prime: u64) -> Self {
        FiniteField { prime }
    }

    fn element(&self, value: u64) -> FiniteFieldElement {
        FiniteFieldElement::new(value, *self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteFieldElement {
    value: u64,
    field: FiniteField,
}

/// TODO: consider what to do when u64 overflows
impl FiniteFieldElement {
    fn new(value: u64, field: FiniteField) -> Self {
        let value_mod = value % field.prime;
        FiniteFieldElement {
            value: value_mod,
            field,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        assert_eq!(self.field.prime, other.field.prime);
        let new_value = (self.value + other.value) % self.field.prime;
        FiniteFieldElement::new(new_value, self.field)
    }

    pub fn subtract(&self, other: Self) -> Self {
        assert_eq!(self.field.prime, other.field.prime);
        // Add prime (first) to make sure the value stays positive
        let new_value = (self.value + self.field.prime - other.value) % self.field.prime;
        FiniteFieldElement::new(new_value, self.field)
    }

    pub fn multiply(&self, other: Self) -> Self {
        assert_eq!(self.field.prime, other.field.prime);
        let new_value = (self.value * other.value) % self.field.prime;
        FiniteFieldElement::new(new_value, self.field)
    }

    pub fn pow(&self, exponent: u64) -> Self {
        let mut result = FiniteFieldElement::new(1, self.field);
        for _ in 0..exponent {
            result = result.multiply(*self);
        }
        result
    }

    pub fn hash(&self) -> u64 {
        hashing::hash(&self.value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let field: FiniteField = FiniteField::new(5);
        let elem: FiniteFieldElement = FiniteFieldElement::new(4, field);

        assert_eq!(elem.value, 4);
        assert_eq!(elem.field.prime, 5);
    }

    #[test]
    fn add() {
        let f: FiniteField = FiniteField::new(5);

        assert_eq!(create(1, f).add(create(3, f)).value, 4);
        assert_eq!(create(4, f).add(create(3, f)).value, 2);
        assert_eq!(create(14, f).add(create(3, f)).value, 2);
        assert_eq!(create(4, f).add(create(14, f)).value, 3);
    }

    #[test]
    fn subtract() {
        let f: FiniteField = FiniteField::new(5);

        assert_eq!(create(4, f).subtract(create(1, f)).value, 3);
        assert_eq!(create(3, f).subtract(create(4, f)).value, 4);
        assert_eq!(create(14, f).subtract(create(3, f)).value, 1);
        assert_eq!(create(4, f).subtract(create(14, f)).value, 0);
    }

    #[test]
    fn multiply() {
        let f: FiniteField = FiniteField::new(5);

        assert_eq!(create(2, f).multiply(create(0, f)).value, 0);
        assert_eq!(create(0, f).multiply(create(2, f)).value, 0);

        assert_eq!(create(2, f).multiply(create(2, f)).value, 4);
        assert_eq!(create(3, f).multiply(create(4, f)).value, 2);
        assert_eq!(create(14, f).multiply(create(3, f)).value, 2);
        assert_eq!(create(4, f).multiply(create(14, f)).value, 1);
    }

    #[test]
    fn pow() {
        let f: FiniteField = FiniteField::new(5);

        assert_eq!(create(2, f).pow(0).value, 1);
        assert_eq!(create(0, f).pow(0).value, 1);

        assert_eq!(create(2, f).pow(1).value, 2);
        assert_eq!(create(1, f).pow(1).value, 1);

        assert_eq!(create(2, f).pow(2).value, 4);
        assert_eq!(create(2, f).pow(3).value, 3);
        assert_eq!(create(3, f).pow(2).value, 4);
    }

    #[test]
    fn hash() {
        let f: FiniteField = FiniteField::new(5);

        assert_ne!(create(1, f).hash(), create(2, f).hash());
        assert_ne!(create(0, f).hash(), create(4, f).hash());

        assert_eq!(create(1, f).hash(), create(6, f).hash());
    }

    /// A silly function to shorten the test lines
    fn create(val: u64, field: FiniteField) -> FiniteFieldElement {
        FiniteFieldElement::new(val, field)
    }
}
