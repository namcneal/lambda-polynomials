use crate::common::*;
use crate::polynomials::*;
use crate::quotients::*;

use duplicate::duplicate_item;

use num_traits::*;
use std::ops::*;


#[duplicate_item(
		Lhs   						Rhs;
	[&Quotient<'a,T,D>]     [&Quotient<'a,T,D>];
	[&Quotient<'a,T,D>]     [Quotient<'a,T,D>];
	[Quotient<'a,T,D>]      [&Quotient<'a,T,D>];
	[Quotient<'a,T,D>]      [Quotient<'a,T,D>];
)]
impl<'a,T:Scalar,const D:usize> Add<Rhs> for Lhs{
	type Output = Quotient<'a,T,D>;

	fn add(self, rhs: Rhs) -> Self::Output {
		let mut cross_multiplied = &self.numerator * &rhs.denominator;
		cross_multiplied = &cross_multiplied + &(&self.denominator * &rhs.numerator);
		Quotient::<'a,T,D>{numerator: cross_multiplied, 
						   denominator: &self.denominator*&rhs.denominator}
	}
}

#[duplicate_item(
		Lhs;
	[&Quotient<'a,T,D>];
	[Quotient<'a,T,D>];
)]
impl<'a,T:Scalar,const D:usize> Neg for Lhs{
	type Output = Quotient<'a,T,D>;

	fn neg(self) -> Self::Output {
		Quotient::<'a,T,D>{numerator: -&self.numerator, 
						denominator: self.denominator.clone()}
	}
}

#[duplicate_item(
	Lhs   						Rhs;
[&Quotient<'a,T,D>]     [&Quotient<'a,T,D>];
[&Quotient<'a,T,D>]     [Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [&Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [Quotient<'a,T,D>];
)]
impl<'a,T:Scalar,const D:usize> Sub<Rhs> for Lhs{
	type Output = Quotient<'a,T,D>;

	fn sub(self, rhs: Rhs) -> Self::Output {
		self + -rhs
	}
}

#[duplicate_item(
	Lhs   						Rhs;
[&Quotient<'a,T,D>]     [&Quotient<'a,T,D>];
[&Quotient<'a,T,D>]     [Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [&Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [Quotient<'a,T,D>];
)]
impl<'a,T:Scalar,const D:usize> Mul<Rhs> for Lhs{
	type Output = Quotient<'a,T,D>;

	fn mul(self, rhs: Rhs) -> Self::Output {
		Quotient::<'a,T,D>{numerator: &self.numerator * &rhs.numerator, 
						   denominator: &self.denominator*&rhs.denominator}
	}
}

#[duplicate_item(
	Lhs   						Rhs;
[&Quotient<'a,T,D>]     [&Quotient<'a,T,D>];
[&Quotient<'a,T,D>]     [Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [&Quotient<'a,T,D>];
[Quotient<'a,T,D>]      [Quotient<'a,T,D>];
)]
impl<'a,T:Scalar,const D:usize> Div<Rhs> for Lhs{
	type Output = Quotient<'a,T,D>;

	fn div(self, rhs: Rhs) -> Self::Output {
		Quotient::<'a,T,D>{numerator: &self.numerator * &rhs.denominator, 
						  denominator: &self.denominator * &rhs.numerator}
	}
}
