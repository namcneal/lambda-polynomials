use crate::common::*;
use crate::variables::*;
use crate::ops::operations::*;

pub (crate) struct Monomial<'a,T: Scalar, const D: usize>{
	coefficient : T,
	variables   : &'a BasisVariables<'a,T,D>,
	exponents   : [u32;D],
	operation   : Operation<'a, T,D>
}

impl<'a, T: Scalar, const D: usize> Monomial<'a,T,D>{
	pub (crate) fn eval(&self, x:&Vector<T,D>) -> T{
		self.operation.0(x)
	}
	
	pub (crate) fn new(variables:&'a BasisVariables<'a,T,D>, exponents:&[u32;D], coefficient:T) -> Monomial<'a,T,D>{
		let mut operation = Operation::<'a,T,D>::constant(coefficient);
		for (i,var) in variables.0.iter().enumerate(){
			for _ in 0..exponents[i]{
				let field = |x:&Vector<T,D>| var.eval(x);
				operation = operation * Operation::<'a,T,D>(Box::new(field))
			}
		}

		return Monomial{coefficient: coefficient,
					    variables:   variables,
						exponents:   *exponents,
						operation:   operation}
	}

	pub (crate) fn directional_derivative(&self, dir:usize) -> Monomial<T, D>{
		let mut new_coefficient : T;
		let mut new_exponents   : [u32;D];
		match self.exponents[dir]{
			0 => { new_coefficient = T::zero();
				   new_exponents   = [0; D];
			},

			k => { new_coefficient = self.coefficient * self.exponents[dir].into();
					    new_exponents = self.exponents.clone();
						new_exponents[dir] -= 1;
			}
		}

		Monomial::<'a,T,D>::new(self.variables, &new_exponents, new_coefficient)
	}
}

pub (crate) struct Polynomial<'a, T: Scalar, const D: usize>(pub (crate) Vec<Monomial<'a,T,D>>);

impl<'a, T: Scalar, const D: usize> Polynomial<'a,T,D>{
	pub fn eval(&self, x:&Vector<T,D>) -> T{
		let mut result = T::zero();
		for monomial in self.0.iter(){
			result = result + monomial.eval(x);
		}
		
		return result
	}

	pub fn directional_derivative(&'a self, dir: usize) -> Polynomial<'a,T,D>{
		let mut differentiated_monomials = Vec::<Monomial<'a,T,D>>::new();
		for monomial in self.0.iter(){
			differentiated_monomials.push(monomial.directional_derivative(dir));
		}

		Polynomial::<'a,T,D>(differentiated_monomials)
	}
}

