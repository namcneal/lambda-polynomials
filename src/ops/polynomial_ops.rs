use crate::common::*;
use crate::polynomials::*;
use crate::ops::operations::*;
use crate::rationals::*;

use std::ops::*;

impl<'a, T: Scalar, const D: usize> Add for &'a Polynomial<'a,T,D>{
	type Output = RationalFunction<'a,T,D>;
	fn add(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x) + rhs.eval(x);
		RationalFunction::<'a,T,D>(Operation::<'a,T,D>(Box::new(field)))
	}
}

impl<'a, T: Scalar, const D: usize> Sub for &'a Polynomial<'a,T,D>{
	type Output = RationalFunction<'a,T,D>;
	fn sub(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x) - rhs.eval(x);
		RationalFunction::<'a,T,D>(Operation::<'a,T,D>(Box::new(field)))
	}
}

impl<'a, T: Scalar, const D: usize> Mul for &'a Polynomial<'a,T,D>{
	type Output = RationalFunction<'a,T,D>;
	fn mul(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x) * rhs.eval(x);
		RationalFunction::<'a,T,D>(Operation::<'a,T,D>(Box::new(field)))
	}
}

impl<'a, T: Scalar, const D: usize> Div for &'a Polynomial<'a,T,D>{
	type Output = RationalFunction<'a,T,D>;
	fn div(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x) / rhs.eval(x);
		RationalFunction::<'a,T,D>(Operation::<'a,T,D>(Box::new(field)))
	}
}
