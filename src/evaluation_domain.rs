use crate::finite_field::{FiniteField, FiniteFieldElement};

/// Minimal, naive evaluation domain: points are [0, 1, ..., n-1] in the field.
#[derive(Debug, Clone)]
pub struct EvaluationDomain {
    pub field: FiniteField,
    pub points: Vec<FiniteFieldElement>,
}

impl EvaluationDomain {
    /// Create a domain with points 0..n-1 over `field`.
    pub fn new_linear(field: FiniteField, n: usize) -> Self {
        assert!(n > 0);
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            points.push(FiniteFieldElement::new_fielded(i as i128, field));
        }
        EvaluationDomain { field, points }
    }

    /// Number of points in the domain.
    pub fn size(&self) -> usize {
        self.points.len()
    }

    /// i-th point in the domain.
    pub fn element(&self, i: usize) -> FiniteFieldElement {
        self.points[i]
    }

    /// Vanishing polynomial Z_H(x) = ∏(x - a_i) over all domain points a_i.
    /// This is O(n) per evaluation; fine for tiny, educational setups.
    pub fn evaluate_vanishing(&self, x: FiniteFieldElement) -> FiniteFieldElement {
        let mut acc = FiniteFieldElement::new_fielded(1, self.field);
        for a in &self.points {
            acc = acc.multiply(x.subtract(*a));
        }
        acc
    }
}
