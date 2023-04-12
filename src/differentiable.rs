use crate::common::*;
use crate::polynomials::{Monomial, Polynomial};
use crate::quotients::{Quotient};
use array_init::array_init;

use duplicate::duplicate_item;

pub (crate) trait Differentiable<'b,T: Scalar, const D:usize>{
	type CoDomain;

	fn zero_output(&self) -> Self::CoDomain;

	fn eval(&self, x:&Vector<T,D>) -> Self::CoDomain; 

	fn partially_differentiate(&self, i:usize) -> Self;

	fn jacobian(&self, x:&Vector<T,D>) -> [Self::CoDomain; D] where Self: Sized{
		let mut jacobian : [Self::CoDomain; D] = array_init(|_| Self::zero_output(&self));

		for i in 0..D{
			let partial_d = self.clone().partially_differentiate(i);

			jacobian[i] = partial_d.eval(x);
		}
		
		jacobian.try_into().unwrap()
	}
}

impl<'a,'b,T: Scalar+'static, const D: usize> Differentiable<'b,T,D> for Monomial<'a,T,D>{
	type CoDomain = T;

	fn zero_output(&self) -> Self::CoDomain {
		T::zero()
	}

	fn eval(&self, x:&Vector<T,D>) -> T {
		self.eval(x)
	}

	fn partially_differentiate(&self, i:usize) -> Self{
		self.directional_derivative(i)
	}
}

impl<'a,'b,T: Scalar, const D: usize> Differentiable<'b,T,D> for Polynomial<'a,T,D>{
	type CoDomain = T;

	fn zero_output(&self) -> Self::CoDomain {
		T::zero()
	}
	
	fn eval(&self, x:&Vector<T,D>) -> T {
		self.eval(x)
	}

	fn partially_differentiate(&self, i:usize) -> Self{
		self.directional_derivative(i)
	}
}

impl<'a,'b,T: Scalar, const D: usize> Differentiable<'b,T,D> for Quotient<'a,T,D>{
	type CoDomain = T;

	fn zero_output(&self) -> Self::CoDomain {
		T::zero()
	}
	
	fn eval(&self, x:&Vector<T,D>) -> T {
		self.eval(x)
	}

	fn partially_differentiate(&self, i:usize) -> Self {
		let mut d_numerator = self.numerator.clone(); 
		let mut d_denominator = self.denominator.clone();
		d_numerator.partially_differentiate(i);
		d_denominator.partially_differentiate(i);

		let mut numerator = &self.denominator * d_numerator;
		numerator = &numerator - &(&self.numerator * d_denominator);

		let mut denominator = &self.denominator * &self.denominator;

		numerator.combine_like_monomials();
		denominator.combine_like_monomials();
		Quotient::<T,D>{numerator: numerator, denominator:denominator}
	}
}


#[duplicate_item(
    dim                      object;
  [ ndarray::Ix0 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix1 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix2 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix3 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix4 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix5 ]    [Polynomial<'a,T,D>];
  [ ndarray::Ix6 ]    [Polynomial<'a,T,D>];
  [ ndarray::IxDyn ]  [Polynomial<'a,T,D>];

  [ ndarray::Ix0 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix1 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix2 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix3 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix4 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix5 ]    [Quotient<'a,T,D>];
  [ ndarray::Ix6 ]    [Quotient<'a,T,D>];
  [ ndarray::IxDyn ]  [Quotient<'a,T,D>];


)]
impl<'a,'b,T:Scalar,const D:usize> Differentiable<'b,T,D> for ndarray::Array<object, dim>{
	type CoDomain = ndarray::Array<T,dim>;

	fn zero_output(&self) -> Self::CoDomain {
		ndarray::Array::<T,dim>::zeros(self.raw_dim())
	}
	
	fn eval(&self, x:&Vector<T,D>) -> Self::CoDomain {
		self.map(|el| el.eval(x))
	}

	fn partially_differentiate(&self, i:usize) -> Self{
		let mut differentiated = self.clone();
		for el in differentiated.iter_mut(){
			*el = (*el).partially_differentiate(i);
		}

		return differentiated
	}
}



