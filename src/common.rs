use std::ops::Neg;
use std::fmt::Debug;
use num_traits::Num;

pub (crate) trait Scalar: Sized + Copy + Clone + Debug + ToString +
						  Num + Neg<Output=Self> +
						  From<u8> + From<f32> + From<f64>{}


impl Scalar for f64 {}

pub (crate) type Vector<T, const D: usize> = [T;D];
pub (crate) type ScalarField<'a, T, const D: usize> = dyn Fn(&Vector<T,D>) -> T + 'a;
