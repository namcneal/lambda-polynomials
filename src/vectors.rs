use crate::common::*;
use crate::polynomials::*;
use crate::quotients::*;

use ndarray::{arr1, Array1};
#[derive(Debug, Clone)]
pub (crate) struct PolyVector<'a, T: Scalar+'static, const D: usize>(pub (crate) Array1<Polynomial<'a,T,D>>);

#[derive(Debug, Clone)]
pub (crate) struct QuotientVector<'a, T: Scalar+'static, const D: usize>(pub (crate) Array1<Quotient<'a,T,D>>);

impl<'a, T:Scalar, const D: usize> From<PolyVector<'a,T,D>> for QuotientVector<'a,T,D>{
	fn from(polyvec: PolyVector<'a,T,D>) -> Self {
		QuotientVector::<'a,T,D>(arr1(
			polyvec.0.iter()
			.map(|poly| Quotient::<'a,T,D>::from(poly.clone()))
			.collect::<Vec<Quotient<'a,T,D>>>()
			.as_slice()
		))
	}	
}

impl<'a, T:Scalar, const D:usize> QuotientVector<'a,T,D>{
	fn standard_basis_vector(ith:usize) -> QuotientVector<'a,T,D>{
		let mut vector = Vec::<Quotient<'a,T,D>>::new();
		for k in 0..D{
			if k == ith{
				vector.push(Quotient::<'a,T,D>::from(Polynomial::<'a,T,D>::constant(T::one())));
			} else{
				vector.push(Quotient::<'a,T,D>::from(Polynomial::<'a,T,D>::constant(T::zero())));
			}
		}

		QuotientVector::<'a,T,D>(arr1(vector.as_slice()))
	}

	fn standard_basis() -> Vec<QuotientVector<'a,T,D>>{
		let mut basis = Vec::<QuotientVector<'a,T,D>>::new();
		for k in 0..D{
			basis.push(Self::standard_basis_vector(k));
		}

		basis
	}

	fn inner(u:&QuotientVector<'a,T,D>, v:&QuotientVector<'a,T,D>) -> Quotient<'a,T,D>{

		let mut inner = Quotient::<'a,T,D>::constant(T::zero());
		for i in 0..D{
			// println!("{:?}, {:?}", &inner, &(u.0[i].clone() * v.0[i].clone()));
			inner = &inner + &(u.0[i].clone() * v.0[i].clone());
		}

		// println!("{:?}", inner);

		return inner;
	}

	fn proj(onto:&QuotientVector<'a,T,D>, of:&QuotientVector<'a,T,D>) -> QuotientVector<'a,T,D>{
		let u = onto;
		let v = of;
		let numerator = Self::inner(v,u);
		let denominator = Self::inner(u,u);

		let projection : Vec<Quotient<'a,T,D>> = u.0.iter().map(|poly| (&numerator * poly) / &denominator).collect();
		QuotientVector::<'a,T,D>(arr1(projection.as_slice()))
	}

	pub (crate) fn graham_schmidt_with_standard_basis(initial_vector:QuotientVector<'a,T,D>) -> Vec<QuotientVector<'a,T,D>>{
		let original_basis = Self::standard_basis();
		let mut v = original_basis;
		v[0] = initial_vector; 
		
		let mut u = Vec::<QuotientVector<'a,T,D>>::new();
		u.push(QuotientVector::<'a,T,D>::from(v[0].clone()));

		for k in 1..D{
			let mut uk = v[k].clone();
			for ui in u.iter(){
				// println!("{:?}\n\n", Self::proj(&ui,&v[k]).0);

				uk.0 = uk.0 - Self::proj(&ui,&v[k]).0;
			}

			u.push(uk);
		}

		return u
	}
}