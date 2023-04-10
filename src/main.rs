mod common;
mod copyable_fn;
mod polynomials;
mod ops;
mod differentiable;
mod random;
mod printing;

use polynomials::*;
use differentiable::*;
use random::*;

use ndarray::arr1;

#[derive(Debug, Clone)]
struct Coframe<T>{
    axis : ndarray::Array1<T>
}

fn main() {
    const dimension : usize = 3;
    const D         : usize = dimension;
    let seed = 1;
    let mean = 0.0;
    let std = 1.0;
    let max_power : u8 = 3;
    let max_num_monomials = 5;

    let mut axis_elements = Vec::<Polynomial<f64, dimension>>::new();
    for i in 0..dimension{
        let element_expansion = random_polynomial::<f64,dimension>(seed, mean, std, max_power, max_num_monomials);
        axis_elements.push(element_expansion);
    }

    let coframe = Coframe{axis:arr1(&axis_elements)};
    
    // let x : [f64; D] = [5.0,2.0,4.0];

    // let arr  = ndarray::array![polynomial.clone(),polynomial.clone()];

    

    // let result = &coframe.axis.jacobian(&x);


    // let t = basis.coordinate(0);
    // let x = basis.coordinate(1);

    println!("{:?}", coframe);

    return ();
}
