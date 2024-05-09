use crate::finite_field::{FiniteField, FiniteFieldElement};
use crate::merkle_tree::MerkleTree;
use crate::number::modulo_multiply;
use crate::polynomial::interpolate::lagrange_interpolation;
use crate::polynomial::polynomial::Polynomial;
use crate::trace::Trace;

/// Pretend these are sent to the verifier at intervals
pub struct Commitments {
    lde_commitment: FiniteFieldElement,
}

pub fn prove(trace: Trace, field: FiniteField, generator: i128) -> Commitments {
    let g_elem = FiniteFieldElement::new_fielded(generator, field);
    //  let poly_g = Polynomial::new([g_elem.pow(3 * 2_i128.pow(20)).value].to_vec());
    let g = g_elem.pow(3 * 2_i128.pow(20));

    // LDE
    let f = get_poly(&trace, generator, field);
    let eval_domain = get_eval_domain(g_elem);

    let mut evaluations: Vec<FiniteFieldElement> = vec![];
    for i in eval_domain.iter() {
        evaluations.push(f.evaluate(*i));
    }
    let mut tree = MerkleTree::new();
    tree.build(&evaluations);
    // LDE Commitment "done"

    // Constraints
    let main_function: Polynomial = Polynomial::new(trace.trace[0].clone());
    let poly_one = Polynomial::new([1].to_vec());
    let poly_zero = Polynomial::new([0].to_vec());
    let poly_x = Polynomial::new([0, 1].to_vec());

    let mut poly_1022_coeffs = vec![0; 1022];
    poly_1022_coeffs.push(1);
    let poly_1022 = Polynomial::new(poly_1022_coeffs);

    let poly_first = main_function
        .clone()
        .sub(&poly_one)
        .div(&poly_x.clone().sub(&poly_one));

    let poly_second = main_function
        .clone()
        .sub(&Polynomial::new([2338775057].to_vec()))
        .div(
            &poly_x
                .clone()
                .sub(&Polynomial::new([g_elem.pow(1022).value].to_vec())),
        );

    let part_one: Polynomial = f.compose(poly_x.clone().multiply_scalar(g.pow(2).value));
    let part_two: Polynomial =
        poly_zero.sub(&f.compose(poly_x.clone().multiply_scalar(g.value)).pow(2));
    let part_three: Polynomial = poly_zero.sub(&f.compose(poly_x.clone()).pow(2));
    let numerator: Polynomial = part_one.add(&part_two).add(&part_three);

    let mut poly_1024_coeffs = vec![0; 1024];
    poly_1024_coeffs.push(1);
    let poly_1024 = Polynomial::new(poly_1024_coeffs);

    let part_one = poly_1024.sub(&poly_one);
    let part_two = poly_x
        .clone()
        .sub(&Polynomial::new([g_elem.pow(1021).value].to_vec()));
    let part_three = poly_x.sub(&Polynomial::new([g_elem.pow(1022).value].to_vec()));
    let part_four = poly_x.sub(&Polynomial::new([g_elem.pow(1023).value].to_vec()));
    let (divisor, _) = part_one.div(&part_two.multiply(&part_three).multiply(&part_four));

    let poly_third = numerator.div(&divisor);

    // let numer_2: Polynomial = f(x() * g).pow(2) * FieldElement::new((-1 + FieldElement::k_modulus() as i128) as usize);

    Commitments {
        lde_commitment: FiniteFieldElement::new_fielded(tree.root().unwrap(), field),
    }
}

fn get_eval_domain(generator: FiniteFieldElement) -> Vec<FiniteFieldElement> {
    // Adjusted from https://github.com/lambdaclass/STARK101-rs/blob/main/part1.ipynb
    let exp = (2_i128.pow(30) * 3) / 8192;
    let h = generator.pow(exp);
    let H: Vec<FiniteFieldElement> = (0..8192).into_iter().map(|i| h.pow(i)).collect();
    let eval_domain: Vec<FiniteFieldElement> =
        H.into_iter().map(|x| generator.multiply(x)).collect();
    eval_domain
}

fn get_poly(trace: &Trace, generator: i128, field: FiniteField) -> Polynomial {
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
         */

        let mut results = vec![];
        evaluate_sq_fibo(1, 3141592, 3221225473, &mut results, 0, 102);

        let trace = Trace::new(vec![results]);
        let field = FiniteField::new(3221225473);
        prove(trace, field, 5);
    }
}
