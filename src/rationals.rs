use crate::common::*;
use crate::ops::operations::*;
use crate::polynomials::*;

pub (crate) struct RationalFunction<'a, T: Scalar, const D: usize>(pub (crate) Operation<'a,T,D>);

impl<'a, T: Scalar, const D: usize> RationalFunction<'a,T,D>{
	pub (crate) fn eval(&self, x:&Vector<T,D>) -> T{
		self.0.0(x)
	}
}

pub (crate) struct RationalVector<'a, T:Scalar, const D: usize>([RationalFunction<'a,T,D>; D]);