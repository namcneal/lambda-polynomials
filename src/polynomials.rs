use crate::common::*;
use crate::ops::operations::*;
use std::collections::HashMap;
use num_traits::abs;



pub (crate) struct Monomial<'a,T: Scalar, const D: usize>{
	pub (crate) coefficient : T,
	pub (crate) exponents   : [u8;D],
	pub (crate) operation   : Operation<'a,T,D>
}

impl<'a,T: Scalar+'static, const D: usize> Monomial<'a,T,D>{
	pub (crate) fn eval(&self, x:&Vector<T,D>) -> T{
		self.coefficient * self.operation.0(x)
	}
	
	pub (crate) fn new(exponents:&[u8;D], coefficient:T) -> Self{
		let mut operation = Operation::<T,D>::one();
		
		for (i,exp) in exponents.iter().enumerate(){
			for _ in 0..*exp{
				operation = operation * Operation::<T,D>(Box::new(move |x:&Vector<T,D>| x[i]))
			}
		}

		return Monomial{coefficient: coefficient,
						exponents:   *exponents,
						operation:   operation}
	}

	pub (crate) fn directional_derivative(&self, dir:usize) -> Self {
		let new_coefficient : T;
		let mut new_exponents   : [u8;D];
		match self.exponents[dir]{
			0 => { new_coefficient = T::zero();
				   new_exponents   = [0; D];
			},

			k => { new_coefficient = self.coefficient * k.into();
					    new_exponents = self.exponents.clone();
						new_exponents[dir] -= 1;
			}
		}

		Monomial::<T,D>::new(&new_exponents, new_coefficient)
	}

	pub fn combine_like_monomials(monomials:&mut Vec<Monomial<'a,T,D>>) -> Vec<Monomial<'a,T,D>>{
		let mut map = HashMap::<[u8;D], Vec<T>>::new();
		for monomial in monomials{
			match map.get_mut(&monomial.exponents){
				None => {
					map.insert(monomial.exponents, vec![monomial.coefficient]);
				},
				Some(vector) => {
					vector.push(monomial.coefficient);
				}
			}
		}

		let epsilon = 1e-16;
		let mut combined_monomials = Vec::<Monomial<'a,T,D>>::new();
		for (exponent, coefficients) in map.iter(){
			let summed_coefficient : T = (*coefficients).iter().fold(T::zero(), |acc,x| acc + *x);
			let monomial = Monomial::<'a,T,D>::new(exponent, summed_coefficient);
			if abs(monomial.coefficient) > <T as From<f64>>::from(epsilon){
				combined_monomials.push(monomial);
			}
				
		}

		return combined_monomials;
	}
}

impl<'a,T:Scalar+'static,const D: usize> Clone for Monomial<'a,T,D>{
	fn clone(&self) -> Self {
		Monomial::<T,D>::new(&self.exponents, self.coefficient)
	}
}

#[repr(C)]
#[derive(Clone)]
pub (crate) struct Polynomial<'a,T: Scalar+'static, const D: usize>{
	pub (crate) constant  : T,
	pub (crate) monomials : Vec<Monomial<'a,T,D>>,
}

impl<'a,T: Scalar+'static, const D: usize> Polynomial<'a,T,D>{
	pub (crate) fn constant(value:T) -> Polynomial<'a,T,D>{
		Polynomial::<'a,T,D>{constant: value, monomials:Vec::<Monomial<'a,T,D>>::new()}
	}
	
	pub fn eval(&self, x:&Vector<T,D>) -> T{
		let mut result = self.constant;
		for monomial in self.monomials(){
			result = result + monomial.eval(x);
		}
		
		return result
	}

	pub (crate) fn monomials(&self) -> impl Iterator<Item = &Monomial<'a,T,D>>{
		self.monomials.iter()
	}	
	
	pub fn directional_derivative(&self, dir: usize) -> Self{
		let mut differentiated_monomials = Vec::<Monomial<T,D>>::new();
		for monomial in self.monomials(){
			differentiated_monomials.push(monomial.clone().directional_derivative(dir));
		}

		let mut poly = Polynomial::<T,D>{constant: T::zero(), monomials:differentiated_monomials};
		poly.combine_like_monomials();
		return poly

	}

	pub fn combine_like_monomials(&mut self){
		let combined_monomials = Monomial::<'a,T,D>::combine_like_monomials(&mut self.monomials);
		self.monomials = combined_monomials;
	}
}

