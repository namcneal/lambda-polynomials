use crate::common::*;
use crate::polynomials::*;

#[derive(Debug,Clone)]
pub (crate) struct Quotient<'a,T:Scalar+'static, const D: usize>{
	pub (crate) numerator   : Polynomial<'a,T,D>,
	pub (crate) denominator : Polynomial<'a,T,D>,
}

impl<'a,T: Scalar, const D: usize> From<Polynomial<'a,T,D>> for Quotient<'a,T,D>{
	fn from(poly: Polynomial<'a,T,D>) -> Self {
		let no_monomials = Vec::<Monomial<'a,T,D>>::new();
		Quotient::<T,D>{numerator: poly, 
						denominator:Polynomial::<T,D>{constant: T::one(), monomials:no_monomials}
		}
	}
}

impl<'a,T: Scalar, const D: usize> Quotient<'a,T,D>{
	pub (crate) fn combine_like_monomials(&mut self){
	    self.numerator.combine_like_monomials();
		self.denominator.combine_like_monomials();

	}

	pub (crate) fn constant(value:T) -> Self{
		Quotient::<T,D>::from(Polynomial::<'a,T,D>::constant(value))
	}
	
	pub (crate) fn eval(&self, x:&Vector<T,D>) -> T{
		self.numerator.eval(x) / self.denominator.eval(x)
	}
}