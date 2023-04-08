use crate::common::*;
use crate::variables::*;
use crate::ops::operations::*;

use std::ops::*;

impl<'a, T: Scalar, const D: usize> Add for &'a Variable<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)+rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Sub for &'a Variable<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn sub(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)-rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Mul for &'a Variable<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)*rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Div for &'a Variable<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn div(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)/rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

