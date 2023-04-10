use crate::common::*;
use crate::polynomials::*;

use std::ops::Mul;

impl<'a,T:Scalar+'static, const D:usize> Mul for &Monomial<'a,T,D>{
	type Output = Monomial<'a,T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		let new_coefficient = self.coefficient * rhs.coefficient;
		let new_exponents: [u8; D] = self.exponents.iter().zip(rhs.exponents.iter())
																			 .map(|(a,b)| a+b)
																			 .collect::<Vec<u8>>().try_into().unwrap();
	
		Monomial::<'a,T,D>::new(&new_exponents,new_coefficient)
	}
}

impl<'a,T:Scalar+'static, const D:usize> Mul<T> for &Monomial<'a,T,D>{
	type Output = Monomial<'a,T,D>;

	fn mul(self, rhs: T) -> Self::Output {
		let new_coefficient = self.coefficient * rhs;
	
		Monomial::<'a,T,D>::new(&self.exponents,new_coefficient)
	}
}


impl<'a,T:Scalar+'static, const D:usize> Mul for Monomial<'a,T,D>{
	type Output = Monomial<'a,T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		&self * &rhs
	}
}

impl<'a,T:Scalar+'static, const D:usize> Mul<T> for Monomial<'a,T,D>{
	type Output = Monomial<'a,T,D>;

	fn mul(self, rhs: T) -> Self::Output {
		&self * rhs
	}
}