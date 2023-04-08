mod common;
mod copyable_fn;
mod variables;
mod polynomials;
mod ops;

use variables::*;
use polynomials::*;

fn main() {
    const dimension : usize = 3;
    const D : usize         = dimension;

    let basis = BasisVariables::<f64, D>::standard_basis();

    let monomial_exp : [u32; D] = [1,2,3];
    let monomial = Monomial::new(&basis, &monomial_exp, 10.0);
    let mut polynomial = Polynomial(vec![monomial]);

    let test : [f64; D] = [5.0,2.0,4.0];

    // let t = basis.coordinate(0);
    // let x = basis.coordinate(1);

    println!("{:?}", &polynomial.eval(&test));
}
