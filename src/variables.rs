use crate::common::*;
use std::{ops::*, marker::PhantomData};
use array_init::array_init;

pub (crate) struct Variable<'a, T: Scalar, const D: usize>{
    index    : usize,
    projector: Box<Coordinate<'a,T,D>>,
}

impl<'a, T: Scalar, const D: usize> Variable<'a,T,D>{
    fn uninit() -> Variable<'a, T,D>{
        Variable::<T,D>{index: usize::MAX, 
                           projector: Box::new(|x:&Vector<T,D>| T::zero())
        }
    }

    fn ith(i: usize) -> Variable<'a,T,D>{
        Variable::<T,D>{index: i, 
                           projector: Box::new(move |x:&Vector<T,D>| x[i])
        }
    }

    pub (crate) fn eval(&self, x: &Vector<T,D>) -> T{
        (*self.projector)(x)
    }
}


pub (crate) struct BasisVariables<'a,T: Scalar, const D: usize>(pub (crate) [Variable<'a,T,D>; D]);

impl<'a,T: Scalar, const D: usize> BasisVariables<'a,T,D>{
    pub (crate) fn standard_basis() -> BasisVariables<'a,T,D>{
        let mut variables : [Variable<T,D>; D] = array_init(|_| Variable::<T,D>::uninit());
        for idx in 0..variables.len(){
            variables[idx].index = idx;
            
            let i = (&variables[idx]).index;
            variables[idx].projector = Box::new(move |x:&Vector<T,D>| x[i]);
        }
        return BasisVariables::<T,D>(variables);        
    }

    pub fn coordinate(&self, i:usize) -> &Variable<T,D>{
        &self.0[i]
    }
}



// impl<'a,T,const D: usize> Mul for &Variable<'a,T,D> 
//     where T: Scalar,
// {
//        type Output = &'a dyn Fn(&Vector<T,D>) -> T;

//     fn mul(self, rhs: Self) -> Self::Output {
//         &|x:&Vector<T,D>| (*self.projector)(x) * (*rhs.projector)(x)
//     }
// }
