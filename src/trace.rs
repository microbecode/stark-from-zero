use core::panic;

/// First dimension is rows, second is columns. All rows have to have the same amount of columns
pub struct Trace {
    trace: Vec<Vec<u64>>,
}

impl Trace {
    pub fn new(trace: Vec<Vec<u64>>) -> Self {
        let first_inner_length = trace.first().map_or(0, |inner| inner.len());
        let all_same_length = trace.iter().all(|inner| inner.len() == first_inner_length);
        if !all_same_length {
            panic!("wrong trace format");
        }

        Trace { trace }
    }

    pub fn num_of_columns(&self) -> u64 {
        self.trace[0].len() as u64
    }
}
