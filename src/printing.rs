use crate::polynomials::*;
use crate::common::*;
use std::fmt;


const SUBSCRIPT : [char; 10] = ['₀','₁','₂','₃','₄','₅','₆','₇','₈','₉'];
const SUPSCRIPT : [char; 10] = ['⁰','¹','²','³','⁴','⁵','⁶','⁷','⁸','⁹'];


enum Script{
    Sub,
    Sup
}
fn num_to_xxscript(number:u32, script:Script) -> String{
    let char_set = match script{
        Script::Sub => SUBSCRIPT,
        Script::Sup => SUPSCRIPT
    };

    number.to_string()
          .chars()
          .map(|c| c.to_digit(10).unwrap())
          .map(|idx| char_set[idx as usize])
          .collect()
}

impl<'a,T: Scalar, const D: usize> fmt::Debug for Monomial<'a,T,D>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		let mut out = self.coefficient.to_string();
        out.push_str("\u{2009}");

        let x = "x";
        for (i,ex) in self.exponents.iter().enumerate(){
            if *ex == 0{
                continue;
            }
            out.push_str(x);
            out.push_str(&num_to_xxscript(i as u32, Script::Sub));
            out.push_str(&num_to_xxscript(*ex as u32, Script::Sup));
        }

        write!(f, "{}", out)
    }
}


impl<'a,T: Scalar, const D: usize> fmt::Debug for Polynomial<'a,T,D>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.constant);
        write!(f, " + ");


        for (i, monomial) in self.monomials().enumerate(){
            monomial.fmt(f);
            

            if i < self.monomials.len()-1{
                write!(f, "{}", " + ");
            }

		}

        Ok(())
	}
}
