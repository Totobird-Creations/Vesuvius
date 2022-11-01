use std::collections::HashMap;
use std::sync::{
    Mutex,
    MutexGuard
};
use std::str::FromStr;

use num_bigint::BigInt;
use num_bigfloat::BigFloat;

use crate::parser::node::*;
use crate::run::types::*;


// pre_verify : Define that the name of the value is taken.
// mid_verify : Convert all type descriptors to value types.
// verify     : Check contents for errors.


impl Program {
    pub fn verify(&self) -> Value {
        
    }
}


impl Declaration {
    pub fn verify(&self) -> Value {
        
    }
}


impl Statement {
    pub fn verify(&self) -> Value {
        
    }
}


impl Expression {
    pub fn verify(&self) -> Value {
        return match (self) {

            Self::EqualsOperation(left, right) => {
                let lval = left.verify();
                let rval = right.verify();
                if (lval.matches_type(&rval)) {
                    lval.equals(&rval)
                } else {
                    // TODO : PROPER ERROR
                    panic!("Can not compare two values of different type.")
                }
            }

        }
    }
}


impl Atom {
    pub fn verify(&self) -> Value {
        return match (self) {

            Self::Literal(lit) => lit.verify(),
            
            Self::Expression(expr) => expr.verify(),

            Self::If(ifs, els) => {
                todo!();
            }
            
        }
    }
}


impl Literal {
    pub fn verify(&self) -> Value {
        return match (self) {

            Self::Int(val) => Value::Int(ValConstr::Eq(
                BigInt::from_str(val).unwrap()
            )),

            Self::Float(int, dec) => Value::Float(ValConstr::Eq(
                BigFloat::from_str(&format!("{}.{}", int, dec)).unwrap()
            )),

            Self::Identifier(name) => {
                todo!();
            }

        }
    }
}