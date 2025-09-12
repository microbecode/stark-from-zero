use crate::finite_field::FiniteFieldElement;
use core::panic;

/// Trace represents computational steps in a STARK proof.
/// First dimension is rows (time steps), second is columns (state variables).
/// All rows must have the same number of columns.
pub struct Trace {
    pub trace: Vec<Vec<i128>>,
    pub num_columns: usize,
}

impl Trace {
    /// Create a new trace with validation
    pub fn new(trace: Vec<Vec<i128>>) -> Self {
        if trace.is_empty() {
            panic!("Trace cannot be empty");
        }

        let num_columns = trace[0].len();
        if num_columns == 0 {
            panic!("Trace rows cannot be empty");
        }

        // Validate all rows have the same number of columns
        for (i, row) in trace.iter().enumerate() {
            if row.len() != num_columns {
                panic!(
                    "Row {} has {} columns, expected {}",
                    i,
                    row.len(),
                    num_columns
                );
            }
        }

        Trace { trace, num_columns }
    }

    /// Number of columns (state variables)
    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    /// Number of rows (computational steps)
    pub fn num_rows(&self) -> usize {
        self.trace.len()
    }

    /// Get value at specific row and column
    pub fn get(&self, row: usize, col: usize) -> Option<i128> {
        self.trace.get(row)?.get(col).copied()
    }

    /// Get entire row
    pub fn get_row(&self, row: usize) -> Option<&Vec<i128>> {
        self.trace.get(row)
    }

    /// Get entire column
    pub fn get_column(&self, col: usize) -> Vec<i128> {
        self.trace.iter().map(|row| row[col]).collect()
    }

    /// Convert to finite field elements
    pub fn to_finite_field_elements(
        &self,
        field: crate::finite_field::FiniteField,
    ) -> Vec<Vec<FiniteFieldElement>> {
        self.trace
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&val| FiniteFieldElement::new_fielded(val, field))
                    .collect()
            })
            .collect()
    }

    /// Create trace from a computation function
    pub fn from_computation<F>(num_steps: usize, num_vars: usize, mut compute: F) -> Self
    where
        F: FnMut(usize, &[i128]) -> Vec<i128>,
    {
        let mut trace: Vec<Vec<i128>> = Vec::with_capacity(num_steps);

        for step in 0..num_steps {
            let prev_state = if step == 0 {
                vec![0; num_vars] // Initial state
            } else {
                trace[step - 1].clone()
            };

            let new_state = compute(step, &prev_state);
            if new_state.len() != num_vars {
                panic!(
                    "Computation function returned {} values, expected {}",
                    new_state.len(),
                    num_vars
                );
            }

            trace.push(new_state);
        }

        Trace::new(trace)
    }
}

/// Example: Fibonacci trace generator
pub mod fibonacci {
    use super::Trace;

    /// Generate a Fibonacci trace: F(n) = F(n-1) + F(n-2)
    /// Columns: [F(n-2), F(n-1), F(n)]
    pub fn generate_fibonacci_trace(num_steps: usize, a: i128, _b: i128) -> Trace {
        Trace::from_computation(num_steps, 3, move |step, prev_state| {
            if step == 0 {
                // Initial state: [0, a, a] - F(0) = a, F(-1) = 0
                vec![0, a, a]
            } else if step == 1 {
                // F(1) = a, F(0) = a, F(-1) = 0
                vec![a, a, a]
            } else {
                // F(n-2), F(n-1), F(n) = F(n-1) + F(n-2)
                let f_n_minus_2 = prev_state[1]; // Previous F(n-1) becomes F(n-2)
                let f_n_minus_1 = prev_state[2]; // Previous F(n) becomes F(n-1)
                let f_n = f_n_minus_1 + f_n_minus_2;
                vec![f_n_minus_2, f_n_minus_1, f_n]
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_creation() {
        let trace_data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let trace = Trace::new(trace_data);

        assert_eq!(trace.num_rows(), 3);
        assert_eq!(trace.num_columns(), 3);
        assert_eq!(trace.get(0, 0), Some(1));
        assert_eq!(trace.get(1, 2), Some(6));
    }

    #[test]
    fn test_fibonacci_trace() {
        let trace = fibonacci::generate_fibonacci_trace(5, 1, 1);

        // First few Fibonacci numbers: 1, 1, 2, 3, 5
        assert_eq!(trace.get(0, 2), Some(1)); // F(0) = 1
        assert_eq!(trace.get(1, 2), Some(1)); // F(1) = 1
        assert_eq!(trace.get(2, 2), Some(2)); // F(2) = 1+1 = 2
        assert_eq!(trace.get(3, 2), Some(3)); // F(3) = 2+1 = 3
        assert_eq!(trace.get(4, 2), Some(5)); // F(4) = 3+2 = 5
    }

    #[test]
    #[should_panic]
    fn test_invalid_trace_different_column_counts() {
        let trace_data = vec![
            vec![1, 2, 3],
            vec![4, 5], // Different number of columns!
        ];
        Trace::new(trace_data);
    }
}
