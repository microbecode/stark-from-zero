use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;

pub fn prove(trace: Trace) {}

#[cfg(test)]
mod tests {
    use crate::{sq_fibo::evaluate_sq_fibo, trace::Trace};

    use super::prove;

    #[test]
    fn prover_test() {
        // Using trace from https://starkware.co/stark-101/

        /*
                Field: 3221225473
                Generator: 5

                LDE:
        1. Create trace
        2. TODO

                 */

        let mut results = vec![];
        evaluate_sq_fibo(1, 3141592, 3221225473, &mut results, 0, 1024);

        let trace = Trace::new(vec![results]);
        prove(trace);
    }
}
