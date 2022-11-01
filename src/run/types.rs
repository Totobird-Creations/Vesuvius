use std::collections::HashMap;

use crate::parser::node::*;


use num_bigint::BigInt;
use num_bigfloat::BigFloat;


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq + PartialOrd> {
    AnyOf (Box<Vec<ValConstr<T>>>), // List of possible constraints
    GtEq  (T),                      // Min (inclusive)
    LtEq  (T),                      // Max (inclusive)
    Eq    (T),                      // Value
    None                            // Unconstrained
}
impl<T : PartialEq + PartialOrd> ValConstr<T> {

    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            ValConstr::AnyOf(vs) => vs.iter().any(|v| v.test(value)),

            ValConstr::GtEq(min) => value >= min,

            ValConstr::LtEq(max) => value <= max,

            ValConstr::Eq(v) => value == v,

            ValConstr::None => true

        }
    }

    // Checks if `self` is a larger bound than `other`.
    // Any value that matches `other` must also match `self`.
    pub fn confines(&self, other : &ValConstr<T>) -> bool {
        return match (self) {

            ValConstr::AnyOf(vs) => vs.iter().all(|v| self.confines(v)),

            ValConstr::GtEq(min) => {
                match (other) {
                    ValConstr::AnyOf (othervs)  => {
                        othervs.iter().all(|otherv| self.confines(otherv))
                    },
                    ValConstr::GtEq  (othermin) => {othermin >= min},
                    ValConstr::LtEq  (_)        => {false},
                    ValConstr::Eq    (otherv)   => {otherv > min},
                    ValConstr::None             => {false}
                }
            },

            ValConstr::LtEq(max) => {
                match (other) {
                    ValConstr::AnyOf (othervs)  => {
                        othervs.iter().all(|otherv| self.confines(otherv))
                    },
                    ValConstr::GtEq  (_)        => {false},
                    ValConstr::LtEq  (othermax) => {othermax <= max},
                    ValConstr::Eq    (otherv)   => {otherv < max},
                    ValConstr::None             => {false}
                }
            },

            ValConstr::Eq(v) => {
                match (other) {
                    ValConstr::AnyOf (othervs)  => {
                        othervs.iter().all(|otherv| self.confines(otherv))
                    },
                    ValConstr::GtEq  (_)        => {false},
                    ValConstr::LtEq  (_)        => {false},
                    ValConstr::Eq    (otherv)   => {otherv == v},
                    ValConstr::None             => {false}
                }
            },

            ValConstr::None => true

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
            else {false}
    }

}
