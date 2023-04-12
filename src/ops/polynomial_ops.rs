use crate::common::*;
use crate::polynomials::*;
use crate::quotients::*;

use std::ops::*;
use std::collections::HashMap;
use num_traits::abs;
use duplicate::duplicate_item;


impl<'a,T: Scalar, const D: usize> Add for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		let new_constant = self.constant + rhs.constant;
		
		let added_monomials = self.monomials()
			.chain(rhs.monomials())
			.map(|mono| mono.clone())
			.collect();

		let mut poly = Polynomial::<'a,T,D>{constant: new_constant, monomials:added_monomials};
		poly.combine_like_monomials();

		return poly;
	}
}

impl<'a,T: Scalar, const D: usize> Add<T> for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;
	fn add(self, rhs: T) -> Self::Output {
		Polynomial::<T,D>{constant: self.constant+rhs, monomials: self.monomials.clone()}
	}
}

impl<'a,T: Scalar, const D: usize> Add<T> for &mut Polynomial<'a,T,D>{
	type Output = ();
	fn add(self, rhs: T) -> Self::Output {
		self.constant = self.constant + rhs;
	}
}

impl<'a,T: Scalar, const D: usize> Neg for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;

	fn neg(self) -> Self::Output {
		let mut negated = Vec::<Monomial<T,D>>::new();
		for monomial in self.monomials(){
			negated.push(Monomial::<T,D>::new(&monomial.exponents, -monomial.coefficient));
		}

		Polynomial::<T,D>{constant: -self.constant, monomials:negated}
	}
}

impl<'a,T: Scalar, const D: usize> Neg for &mut Polynomial<'a,T,D>{
	type Output = ();

	fn neg(self) -> Self::Output {
		-self
	}
}

impl<'a,T: Scalar, const D: usize> Sub for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;
	fn sub(self, rhs: Self) -> Self::Output {
		self + &-rhs
	}
}

impl<'a,T: Scalar, const D: usize> Sub<T> for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;
	fn sub(self, rhs: T) -> Self::Output {
		let neg_rhs = -rhs;
		self + neg_rhs
	}
}


impl<'a,T: Scalar, const D: usize> Mul for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;
	fn mul(self, rhs: Self) -> Self::Output {
		let mut monomials = Vec::<Monomial<T,D>>::new();

		for b in rhs.monomials(){
			monomials.push(b*self.constant);	
		}
		for a in self.monomials(){
			monomials.push(a*rhs.constant);
		}
		for a in self.monomials(){
			for b in rhs.monomials(){
				monomials.push(a*b);

			}
		}

	let mut poly = Polynomial::<T,D>{constant:self.constant*rhs.constant, monomials:monomials};
	poly.combine_like_monomials();

	return poly;
	}
}

impl<'a,T: Scalar, const D: usize> Mul<Polynomial<'a,T,D>> for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;

	fn mul(self, rhs: Polynomial<'a,T,D>) -> Self::Output {
		self * &rhs
	}
}

impl<'a,T: Scalar, const D: usize> Mul<T> for &Polynomial<'a,T,D>{
	type Output = Polynomial<'a,T,D>;

	fn mul(self, rhs: T) -> Self::Output {
		let mut monomials = Vec::<Monomial<'a,T,D>>::new();
		for monomial in monomials.iter_mut(){
			monomial.coefficient = monomial.coefficient * rhs;
		}

		Polynomial::<'a,T,D>{constant: self.constant*rhs, monomials: monomials}
	}
}

impl<'a,T: Scalar, const D: usize> Div for &Polynomial<'a,T,D>{
	type Output = Quotient<'a,T,D>;
	fn div(self, rhs: Self) -> Self::Output {
		Quotient::<T,D>{numerator: (*self).clone(), denominator: (*rhs).clone()}
	}
}

impl<'a,T: Scalar, const D: usize> Div for Polynomial<'a,T,D>{
	type Output = Quotient<'a,T,D>;
	fn div(self, rhs: Self) -> Self::Output {
		Quotient::<T,D>{numerator: self, denominator: rhs}
	}
}
