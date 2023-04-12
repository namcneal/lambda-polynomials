mod common;
mod polynomials;
mod quotients;
mod vectors;
mod ops;
mod differentiable;
mod random;
mod printing;

mod coframes;

use common::*;
use polynomials::*;
use differentiable::*;
use coframes::*;


fn main() {
    const dimension : usize = 3;
    const D         : usize = dimension;
    let seed = 1;
    let mean = 0.0;
    let std = 1.0;
    let max_power : u8 = 1;
    let max_num_monomials = 2;


    let coframe = Coframe::<f64,dimension>::new(seed,mean,std,max_power, max_num_monomials);
    let x : [f64; D] = [0.5, 1.0, 1.5];

    // let arr  = ndarray::array![polynomial.clone(),polynomial.clone()];

    

    let R = coframe.matrix(&x);


    // let t = basis.coordinate(0);
    // let x = basis.coordinate(1);

    // let a = &coframe.axis[0];
    // let b = &coframe.axis[2];
    // println!("{:?}", a);
    // // println!("{:?}", b);
    // println!("{:?}", &(a-a) + &(b-b));

    println!("{:?}", &coframe.matrix.jacobian(&x));

    return ();
}
