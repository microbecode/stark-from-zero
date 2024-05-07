use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::number::modulo_multiply;
use crate::polynomial::interpolate::lagrange_interpolation;
use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;

pub fn prove(trace: Trace, field: FiniteField, generator: i128) {
    // LDE
    let poly = get_poly(trace, generator, field);

    let eval_domain = get_eval_domain(generator);
}

fn get_eval_domain(generator: i128) -> Vec<FiniteFieldElement> {
    let w = FiniteFieldElement::new(generator);
    // Adjusted from https://github.com/lambdaclass/STARK101-rs/blob/main/part1.ipynb
    let exp = (2_i128.pow(30) * 3) / 8192;
    let h = w.pow(exp);
    let H: Vec<FiniteFieldElement> = (0..8192).into_iter().map(|i| h.pow(i)).collect();
    let eval_domain: Vec<FiniteFieldElement> = H.into_iter().map(|x| w.multiply(x)).collect();
    eval_domain
}

fn get_poly(trace: Trace, generator: i128, field: FiniteField) -> Polynomial {
    let mut inputs: Vec<(i128, i128)> = vec![];
    let mut previous_input_g = 1;
    for i in 0..trace.trace[0].len() {
        let first = if i == 0 {
            1 // first entry should be just 1
        } else {
            modulo_multiply(previous_input_g, generator, field.prime)
        };
        previous_input_g = first;
        inputs.push((first, trace.trace[0][i]));
    }
    lagrange_interpolation(&inputs[..])
}

#[cfg(test)]
mod tests {
    use crate::{finite_field::FiniteField, sq_fibo::evaluate_sq_fibo, trace::Trace};

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
        evaluate_sq_fibo(1, 3141592, 3221225473, &mut results, 0, 102);

        let trace = Trace::new(vec![results]);
        let field = FiniteField::new(3221225473);
        prove(trace, field, 5);
    }
}
