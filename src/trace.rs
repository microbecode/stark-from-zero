use core::panic;

/// First dimension is rows, second is columns. All rows have to have the same amount of columns
pub struct Trace {
    pub trace: Vec<Vec<i128>>,
}

impl Trace {
    pub fn new(trace: Vec<Vec<i128>>) -> Self {
        /*      for row in trace.iter() {
            if row.len() != 3 {
                panic!("wrong trace format");
            }
        } */

        Trace { trace }
    }

    pub fn num_of_columns(&self) -> u128 {
        self.trace[0].len() as u128
    }
}
