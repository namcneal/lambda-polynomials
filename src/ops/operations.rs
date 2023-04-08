use crate::common::*;
use crate::variables::*;

use std::ops::*;

pub (crate) struct Operation<'a,T:Scalar, const D:usize>(pub (crate) Box<ScalarField<'a,T,D>>);


impl<'a, T:Scalar+'a, const D: usize> Operation<'a,T,D>{
	pub (crate) fn zero() -> Operation<'a,T,D>{
		let field = |x:&Vector<T,D>| T::zero();
		Operation::<T,D>(Box::new(field))
	}

	pub (crate) fn one() -> Operation<'a,T,D>{
		let field = |x:&Vector<T,D>| T::one();
		Operation::<T,D>(Box::new(field))
	}

	pub (crate) fn constant(constant:T) -> Operation<'a,T,D>{
		let field = move |x:&Vector<T,D>| constant;
		Operation::<T,D>(Box::new(field))
	}

	pub (crate) fn eval(&self, x:&Vector<T,D>) -> T{
		self.0(x)
	}
}

impl<'a, T: Scalar, const D: usize> Add for &'a Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)+rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar +'a, const D: usize> Add<&'a Operation<'a,T,D>> for Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn add(self, rhs: &'a Self) -> Self::Output {
		let field = move |x:&Vector<T,D>| self.eval(x)+rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar +'a, const D: usize> Add for Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		let field = move |x:&Vector<T,D>| self.eval(x)+rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Sub for &'a Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn sub(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)-rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar + 'a, const D: usize> Sub for Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn sub(self, rhs: Self) -> Self::Output {
		let field = move |x:&Vector<T,D>| self.eval(x)-rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Mul for &'a Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)*rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar+'a, const D: usize> Mul for Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		let field = move |x:&Vector<T,D>| self.eval(x)*rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar, const D: usize> Div for &'a Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn div(self, rhs: Self) -> Self::Output {
		let field = |x:&Vector<T,D>| self.eval(x)/rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}

impl<'a, T: Scalar + 'a, const D: usize> Div for Operation<'a,T,D>{
	type Output = Operation<'a,T,D>;

	fn div(self, rhs: Self) -> Self::Output {
		let field = move |x:&Vector<T,D>| self.eval(x)/rhs.eval(x);
		Operation::<T,D>(Box::new(field))
	}
}