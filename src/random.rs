use crate::polynomials::*;
use crate::common::*;

use rand::prelude::*;
use rand_distr::Normal;
use rand::seq::IteratorRandom; // 0.6.1


fn random_monomial<'a,T:Scalar+'static, const D:usize>(rng:&mut impl Rng, mean:f64, std:f64, 
													   max_variables:usize, 
													   max_power:u8)->Monomial<'a,T,D>{
	let normal = Normal::new(mean, std).unwrap();
	
	let num_variables : usize = rng.gen_range(1..=max_variables);

	let variables : Vec<usize> = (0..D).choose_multiple(rng, num_variables);

	let mut exponents : [u8; D] = [0; D];
	for var in variables{
		let power    : u8 = rng.gen_range(1..=max_power);
		exponents[var] = power ;
	}

	let coefficient= <T as From<f64>>::from(normal.sample(rng));

	Monomial::<'a,T,D>::new(&exponents,coefficient)
}

pub (crate) fn random_polynomial<'a,T,const D:usize>(mut rng:&mut impl Rng, mean:f64, std:f64, 
													 max_power:u8, max_num_monomials:u64) -> Polynomial<'a,T,D>
	where T: Scalar + 'static
{
	let mut monomials = Vec::<Monomial<'a,T,D>>::new();	
	let num_monomials = rng.gen_range(0..max_num_monomials);
	for _ in 0..=num_monomials{
		monomials.push(random_monomial(&mut rng, mean, std, D, max_power));
	}

	let normal = Normal::new(mean, std).unwrap();
	let constant= <T as From<f64>>::from(normal.sample(&mut rng));

	Polynomial::<'a,T,D>{constant, monomials:monomials}

}