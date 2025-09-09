use crate::hashing;

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    pub prime: i128,
}

impl FiniteField {
    pub fn new(prime: i128) -> Self {
        FiniteField { prime }
    }

    pub fn element(&self, value: i128) -> FiniteFieldElement {
        FiniteFieldElement::new_fielded(value, *self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteFieldElement {
    pub value: i128,
    pub field: FiniteField,
}

/// TODO: consider what to do when i128 overflows
impl FiniteFieldElement {
    pub const DEFAULT_FIELD_SIZE: i128 = 3 * 2_i128.pow(30) + 1;
    const DEFAULT_FIELD: FiniteField = FiniteField {
        prime: Self::DEFAULT_FIELD_SIZE,
    };

    pub const ZERO: Self = FiniteFieldElement {
        value: 0,
        field: Self::DEFAULT_FIELD,
    };

    pub fn new(value: i128) -> Self {
        let value_mod = value % Self::DEFAULT_FIELD.prime;
        FiniteFieldElement {
            value: value_mod,
            field: Self::DEFAULT_FIELD,
        }
    }

    pub fn new_fielded(value: i128, field: FiniteField) -> Self {
        let value_mod = value % field.prime;
        FiniteFieldElement {
            value: value_mod,
            field,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        let new_value = (self.value + other.value) % self.field.prime;
        FiniteFieldElement::new_fielded(new_value, self.field)
    }

    pub fn subtract(&self, other: Self) -> Self {
        // Add prime (first) to make sure the value stays positive
        let new_value = (self.value + self.field.prime - other.value) % self.field.prime;
        FiniteFieldElement::new_fielded(new_value, self.field)
    }

    pub fn multiply(&self, other: Self) -> Self {
        assert_eq!(self.field.prime, other.field.prime);
        let new_value = (self.value * other.value) % self.field.prime;
        FiniteFieldElement::new_fielded(new_value, self.field)
    }

    pub fn pow(&self, exponent: i128) -> Self {
        // Fast exponentiation by squaring
        let mut result = FiniteFieldElement::new_fielded(1, self.field);
        let mut base = *self;
        let mut exp = exponent;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.multiply(base);
            }
            base = base.multiply(base);
            exp /= 2;
        }
        result
    }

    pub fn inverse(&self) -> Self {
        // Fermat's little theorem: a^(p-2) mod p
        self.pow(self.field.prime - 2)
    }

    pub fn hash(&self) -> i128 {
        hashing::hash(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let field: FiniteField = FiniteField::new(5);
        let elem: FiniteFieldElement = FiniteFieldElement::new_fielded(4, field);

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
        assert_eq!(create(2, f).pow(3).value, 3);
        assert_eq!(create(3, f).pow(2).value, 4);
    }

    #[test]
    fn inverse() {
        let f: FiniteField = FiniteField::new(5);
        assert_eq!(create(2, f).inverse().value, 3); // 2*3=6≡1
        assert_eq!(create(3, f).inverse().value, 2); // 3*2=6≡1
        assert_eq!(create(4, f).inverse().value, 4); // 4*4=16≡1
    }

    #[test]
    fn hash() {
        let f: FiniteField = FiniteField::new(5);

        assert_ne!(create(1, f).hash(), create(2, f).hash());
        assert_ne!(create(0, f).hash(), create(4, f).hash());

        assert_eq!(create(1, f).hash(), create(6, f).hash());
    }

    /// A silly function to shorten the test lines
    fn create(val: i128, field: FiniteField) -> FiniteFieldElement {
        FiniteFieldElement::new_fielded(val, field)
    }
}
