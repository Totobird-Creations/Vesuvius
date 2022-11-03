use std::collections::HashMap;
use std::cmp::Ordering;

use num_bigint::{
    BigInt,
    ToBigInt
};
use num_bigfloat::BigFloat;

use crate::parser::node::*;


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq> {
    None,           // No value matches
    Values(Vec<T>), // A list of possible values
    Full            // Any value matches
}
#[derive(Debug, Clone)]
pub enum ValConstrOrd<T : PartialEq + PartialOrd> {
    None,                           // No value matches
    Ranges(Vec<ValConstrRange<T>>), // A list of ranges
    Full                            // Any value matches
}
#[derive(Debug, Clone)]
pub enum ValConstrRange<T : PartialEq + PartialOrd> {
    Exact(T),
    MinInMaxIn(T, T),
    MinInMaxEx(T, T),
    MinExMaxIn(T, T),
    MinExMaxEx(T, T)
}


#[derive(Debug, Clone)]
pub enum Value {
    Void,

    FuncType(Box<Vec<(String, Value)>>, Box<Option<Value>>, Block),

    Int(ValConstrOrd<ValuePossiblyBigInt>),
    Float(ValConstrOrd<ValuePossiblyBigFloat>),

    Bool(ValConstr<bool>)

}
impl Value {

    pub fn matches_type(&self, other : &Value) -> bool {
        return
            if      (matches!(self, Self::Int   (_) ) && matches!(other, Self::Int   (_) )) {true}
            else if (matches!(self, Self::Float (_) ) && matches!(other, Self::Float (_) )) {true}
            else if (matches!(self, Self::Bool  (_) ) && matches!(other, Self::Bool  (_) )) {true}
            else {false}
    }

    pub fn equals(&self, other : &Value) -> Value {
        return match (self) {
            Self::Int   (l) => match(other) {Self::Int   (r) => {todo!()}},
            Self::Float (l) => match(other) {Self::Float (r) => {todo!()}},
            Self::Bool  (l) => match(other) {Self::Bool  (r) => {todo!()}},
            _ => {Value::Bool(ValConstr::Values(vec![false]))}
        };
    }

}


#[derive(Debug, Clone)]
pub enum ValuePossiblyBigInt {
    Small(i64),
    Big(BigInt)
}
impl PartialEq for ValuePossiblyBigInt {
    fn eq(&self, other : &Self) -> bool {
        match (self) {
            ValuePossiblyBigInt::Small(a) => {match (other) {
                ValuePossiblyBigInt::Small (b) => {a == b}
                ValuePossiblyBigInt::Big   (b) => {&a.to_bigint().unwrap() == b}
            }},
            ValuePossiblyBigInt::Big(a) => {match (other) {
                ValuePossiblyBigInt::Small (b) => {a == &b.to_bigint().unwrap()}
                ValuePossiblyBigInt::Big   (b) => {a == b}
            }}
        }
    }
}
impl PartialOrd for ValuePossiblyBigInt {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        match (self) {
            ValuePossiblyBigInt::Small(a) => {match (other) {
                ValuePossiblyBigInt::Small (b) => {a.partial_cmp(b)}
                ValuePossiblyBigInt::Big   (b) => {a.to_bigint().unwrap().partial_cmp(b)}
            }},
            ValuePossiblyBigInt::Big(a) => {match (other) {
                ValuePossiblyBigInt::Small (b) => {a.partial_cmp(&b.to_bigint().unwrap())}
                ValuePossiblyBigInt::Big   (b) => {a.partial_cmp(b)}
            }}
        }
    }
}


#[derive(Debug, Clone)]
pub enum ValuePossiblyBigFloat {
    Small(f64),
    Big(BigFloat)
}
impl PartialEq for ValuePossiblyBigFloat {
    fn eq(&self, other : &Self) -> bool {
        match (self) {
            ValuePossiblyBigFloat::Small(a) => {match (other) {
                ValuePossiblyBigFloat::Small (b) => {a == b}
                ValuePossiblyBigFloat::Big   (b) => {&BigFloat::from_f64(*a) == b}
            }},
            ValuePossiblyBigFloat::Big(a) => {match (other) {
                ValuePossiblyBigFloat::Small (b) => {a == &BigFloat::from_f64(*b)}
                ValuePossiblyBigFloat::Big   (b) => {a == b}
            }}
        }
    }
}
impl PartialOrd for ValuePossiblyBigFloat {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        match (self) {
            ValuePossiblyBigFloat::Small(a) => {match (other) {
                ValuePossiblyBigFloat::Small (b) => {a.partial_cmp(b)}
                ValuePossiblyBigFloat::Big   (b) => {BigFloat::from_f64(*a).partial_cmp(b)}
            }},
            ValuePossiblyBigFloat::Big(a) => {match (other) {
                ValuePossiblyBigFloat::Small (b) => {a.partial_cmp(&BigFloat::from_f64(*b))}
                ValuePossiblyBigFloat::Big   (b) => {a.partial_cmp(b)}
            }}
        }
    }
}
