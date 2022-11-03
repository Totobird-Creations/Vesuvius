use std::collections::HashMap;

use crate::parser::node::*;


use num_bigint::BigInt;
use num_bigfloat::BigFloat;


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq + PartialOrd> {
    None,               // No Value Matches
    Range(Vec<(T, T)>), // Min, Max Inclusive 
    Full                // Any Value Matches
}
impl<T : PartialEq + PartialOrd> ValConstr<T> {

    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            Self

        }
    }

}


#[derive(Debug, Clone)]
pub enum Value {
    Void,

    FuncType(Box<Vec<(String, Value)>>, Box<Option<Value>>, Block),

    Int(ValConstr<BigInt>),
    Float(ValConstr<BigFloat>),

    Bool(ValConstr<bool>)

}
impl Value {

    pub fn matches_type(&self, other : &Value) -> bool {
        return
            if      (matches!(self, Self::Int(_)) && matches!(other, Self::Int(_))) {true}
            else if (matches!(self, Self::Float(_)) && matches!(other, Self::Float(_))) {true}
            else if (matches!(self, Self::Bool(_)) && matches!(other, Self::Bool(_))) {true}
            else {false}
    }

    pub fn equals(&self, other : &Value) -> Value {
        return
            if      let Self::Int(l) = self && let Self::Int(r) = other {todo!()}
            else if let Self::Float(l) = self && let Self::Float(r) = other {todo!()}
            else if let Self::Bool(l) = self && let Self::Bool(r) = other {todo!()}
            else {Value::Bool(ValConstr::Eq(false))}
    }

}
