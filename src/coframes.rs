use crate::common::*;
use crate::polynomials::*;
use crate::quotients::Quotient;
use crate::vectors::*;
use crate::random::*;

use ndarray::{arr1,arr2};
use ndarray::{Array, Array1, Array2};
use rand::rngs::StdRng;
use rand::SeedableRng;

type QuotientMatrix<'a,T,const D: usize> = Array2<Quotient<'a,T,D>>;

#[derive(Debug, Clone)]
pub (crate) struct Coframe<'a, T: Scalar+'static, const D: usize>{
    pub (crate) axis   : QuotientVector<'a,T,D>,
    pub (crate) matrix : QuotientMatrix<'a,T,D>,
}

struct CoframeGenerationParameters{
    seed:u64, 
    mean:f64, 
    std:f64, 
    max_variable_power:u8, 
    max_num_expansion_monomials:u64
}

impl<'a,T: Scalar, const D: usize> Coframe<'a,T,D>{
    pub fn axis(&self, x:&Vector<T,D>) -> Array1<T>{
        let mut axis = arr1(&[T::zero(); D]);
        for i in 0..D{
            axis[i] = self.axis.0[i].eval(x);
        }

        return axis;
    }

	pub fn matrix(&self, x:&Vector<T,D>) -> Array2<T>{
        let mut matrix = Array2::<T>::zeros((D,D));
        for i in 0..D{
			for j in 0..D{
				matrix[[i,j]] = self.matrix[[i,j]].eval(x);

			}
        }

        return matrix
    }


    pub (crate) fn new(seed:u64, mean:f64, std:f64, max_variable_power:u8, max_num_expansion_monomials:u64)->Coframe<'a,T, D>{
        let random_axis = Self::generate_random_axis(
            CoframeGenerationParameters { seed: seed, mean: mean, std: std, 
                                                                max_variable_power: max_variable_power, 
                                                                max_num_expansion_monomials: max_num_expansion_monomials }); 

		let mut random_axis = QuotientVector::<'a,T,D>::from(PolyVector::<'a,T,D>(random_axis.clone()));
		let orthogonal_basis = QuotientVector::<'a,T,D>::graham_schmidt_with_standard_basis(random_axis.clone());
		let mut orthogonal_matrix = Array::from_elem((D,D), Quotient::<'a,T,D>::constant(T::zero()));
		for i in 0..D{for j in 0..D{
				orthogonal_matrix[[i,j]] = orthogonal_basis[i].0[j].clone();
			}
		}
		
        for quotient in random_axis.0.iter_mut(){
            quotient.combine_like_monomials();
        }
        for quotient in orthogonal_matrix.iter_mut(){
            quotient.combine_like_monomials();
        }
        
        Coframe { axis:random_axis, matrix: orthogonal_matrix}
		
    }
	

    fn generate_random_axis(generation_parameters:CoframeGenerationParameters) -> Array1<Polynomial<'a,T,D>>{
        let params = generation_parameters;
        let p = params;
         
        let mut rng  = StdRng::seed_from_u64(p.seed);

        let mut axis_elements = Vec::<Polynomial<'a,T, D>>::new();
        for _ in 0..D{
            let element_expansion = random_polynomial::<T,D>(&mut rng, p.mean, p.std, p.max_variable_power, p.max_num_expansion_monomials);
            axis_elements.push(element_expansion);
        }
    
        arr1(axis_elements.as_slice())
    }
}