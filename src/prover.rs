use crate::polynomial::{lagrange_interpolation, Polynomial};
use crate::trace::Trace;

pub fn prove(trace: Trace) {
    //let mut polys: Vec<Polynomial> = vec![];
    /*  let mut curr_poly: Polynomial = Polynomial::new(vec![0.0]);
    for row in trace.trace.iter() {
        let input_1 = row[0] as f64;
        let input_2 = row[1] as f64;
        let poly = lagrange_interpolation(&[(input_1, input_2)]);
        curr_poly = curr_poly.add(&poly);
    } */
}

#[cfg(test)]
mod tests {
    use crate::trace::Trace;

    use super::prove;

    fn something() {
        // (x1 + x2)(x2 + w1) with inputs x1 = 5, x2= 6, w1 = 1
        /*

        Trace with input row:
        [5,6,1]
        [5,6,11]
        [6,1,7]
        [11,7,77]

        Trace:
        [5,6,11]
        [6,1,7]
        [11,7,77]

        in1 = 5
        in2 = 6
        in3 = 1
        out = 77

        constraints row-by-row:
        - in1 + in2 = temp1
        - in2 + in3 = temp2
        - temp1 * temp2 = out


        all constraints together: (in1 + in2) * (in2 + in3) = out

        Replaced names with polynomial indexing:
        - p1(x) + p2(x) = p3(x)
        - p1(x + 1) + p2(x + 1) = p3(x + 1)
        - p3(x) * p3(x + 1) = p3(x + 2)




        */
        let trace = Trace::new(vec![vec![5, 6, 11], vec![6, 1, 7], vec![11, 7, 77]]);
        prove(trace);
    }
}
