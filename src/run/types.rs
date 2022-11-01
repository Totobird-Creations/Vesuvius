use std::collections::HashMap;

use crate::parser::node::*;


use num_bigint::BigInt;
use num_bigfloat::BigFloat;


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq + PartialOrd> {
    And  (Box<ValConstr<T>>, Box<ValConstr<T>>),
    Or   (Box<ValConstr<T>>, Box<ValConstr<T>>),
    Not  (Box<ValConstr<T>>),
    Eq   (T),
    Ne   (T),
    Gt   (T),
    GtEq (T),
    Lt   (T),
    LtEq (T),
    Unconstrained
}
impl<T : PartialEq + PartialOrd> ValConstr<T> {

    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            Self::And(a, b) => a.test(value) && b.test(value),

            Self::Or(a, b) => a.test(value) || b.test(value),

            Self::Not(a) => ! a.test(value),

            Self::Eq(a) => value == a,

            Self::Ne(a) => value != a,

            Self::Gt(a) => value > a,

            Self::GtEq(a) => value >= a,

            Self::Lt(a) => value < a,

            Self::LtEq(a) => value <= a,

            Self::Unconstrained => true

        }
    }

    // Checks if `self` is a larger bound than `other`.
    // Any value that matches `other` must also match `self`.
    pub fn confines(&self, other : &ValConstr<T>) -> bool {
        return match (self) {

            Self::And(a, b) => a.confines(other) && b.confines(other),

            Self::Or(a, b) => a.confines(other) || b.confines(other),

            Self::Not(a) => todo!(),

            Self::Eq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a == c,
                    Self::Ne(c)         => false,
                    Self::Gt(c)         => false,
                    Self::GtEq(c)       => false,
                    Self::Lt(c)         => false,
                    Self::LtEq(c)       => false,
                    Self::Unconstrained => false
                }
            },

            Self::Ne(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a != c,
                    Self::Ne(c)         => a == c,
                    Self::Gt(c)         => a <= c,
                    Self::GtEq(c)       => a < c,
                    Self::Lt(c)         => a >= c,
                    Self::LtEq(c)       => a > c,
                    Self::Unconstrained => false
                }
            },

            Self::Gt(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a < c,
                    Self::Ne(c)         => false,
                    Self::Gt(c)         => a <= c,
                    Self::GtEq(c)       => a < c,
                    Self::Lt(c)         => false,
                    Self::LtEq(c)       => false,
                    Self::Unconstrained => false
                }
            },

            Self::GtEq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a <= c,
                    Self::Ne(c)         => false,
                    Self::Gt(c)         => a < c,
                    Self::GtEq(c)       => a <= c,
                    Self::Lt(c)         => false,
                    Self::LtEq(c)       => false,
                    Self::Unconstrained => false
                }
            },

            Self::Lt(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a > c,
                    Self::Ne(c)         => false,
                    Self::Gt(c)         => false,
                    Self::GtEq(c)       => false,
                    Self::Lt(c)         => a >= c,
                    Self::LtEq(c)       => a > c,
                    Self::Unconstrained => false
                }
            },

            Self::LtEq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a >= c,
                    Self::Ne(c)         => false,
                    Self::Gt(c)         => false,
                    Self::GtEq(c)       => false,
                    Self::Lt(c)         => a > c,
                    Self::LtEq(c)       => a >= c,
                    Self::Unconstrained => false
                }
            },

            Self::Unconstrained => true

        }
    }

    // Checks if `self` is a larger bound than `other`.
    // Any value that matches `other` must also match `self`.
    pub fn intersects(&self, other : &ValConstr<T>) -> bool {
        return match (self) {

            Self::And(a, b) => a.confines(other) && b.confines(other),

            Self::Or(a, b) => a.confines(other) || b.confines(other),

            Self::Not(a) => todo!(),

            Self::Eq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a == c,
                    Self::Ne(c)         => a != c,
                    Self::Gt(c)         => a < c,
                    Self::GtEq(c)       => a <= c,
                    Self::Lt(c)         => a > c,
                    Self::LtEq(c)       => a >= c,
                    Self::Unconstrained => true
                }
            },

            Self::Ne(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a != c,
                    Self::Ne(c)         => true,
                    Self::Gt(c)         => true,
                    Self::GtEq(c)       => true,
                    Self::Lt(c)         => true,
                    Self::LtEq(c)       => true,
                    Self::Unconstrained => true
                }
            },

            Self::Gt(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => a < c,
                    Self::Ne(c)         => true,
                    Self::Gt(c)         => true,
                    Self::GtEq(c)       => true,
                    Self::Lt(c)         => true,
                    Self::LtEq(c)       => true,
                    Self::Unconstrained => true
                }
            },

            Self::GtEq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => todo!(),
                    Self::Ne(c)         => todo!(),
                    Self::Gt(c)         => todo!(),
                    Self::GtEq(c)       => todo!(),
                    Self::Lt(c)         => todo!(),
                    Self::LtEq(c)       => todo!(),
                    Self::Unconstrained => todo!()
                }
            },

            Self::Lt(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => todo!(),
                    Self::Ne(c)         => todo!(),
                    Self::Gt(c)         => todo!(),
                    Self::GtEq(c)       => todo!(),
                    Self::Lt(c)         => todo!(),
                    Self::LtEq(c)       => todo!(),
                    Self::Unconstrained => todo!()
                }
            },

            Self::LtEq(a) => {
                match (other) {
                    Self::And(c, d)     => todo!(),
                    Self::Or(c, d)      => todo!(),
                    Self::Not(c)        => todo!(),
                    Self::Eq(c)         => todo!(),
                    Self::Ne(c)         => todo!(),
                    Self::Gt(c)         => todo!(),
                    Self::GtEq(c)       => todo!(),
                    Self::Lt(c)         => todo!(),
                    Self::LtEq(c)       => todo!(),
                    Self::Unconstrained => todo!()
                }
            },

            Self::Unconstrained => true

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
