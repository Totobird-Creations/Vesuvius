use std::collections::HashMap;

use crate::parser::node::*;


use num_bigint::{
    BigInt,
    BigUint
};
use num_bigfloat::BigFloat;


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq + PartialOrd> {
    AnyOf    (Box<Vec<ValConstr<T>>>), // List of possible constraints
    InRange  (T, T),                   // Min (inclusive), Max (exclusive)
    IsValue  (T),                      // Value
    None                               // Unconstrained
}
impl<T : PartialEq + PartialOrd> ValConstr<T> {

    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            ValConstr::AnyOf(vs) => vs.iter().any(|v| v.test(value)),

            ValConstr::InRange(min, max) => value >= min && value < max,

            ValConstr::IsValue(v) => value == v,

            ValConstr::None => true

        }
    }

}



#[derive(Debug, Clone)]
pub enum Value {
    Void,

    FuncType(Box<Vec<(String, Value)>>, Box<Option<Value>>, Block),

    Int(ValConstr<BigInt>),
    Uint(ValConstr<BigUint>),

    Float(BigFloat),
    Ufloat(BigFloat)

}
pub fn into_value_type(typ : &TypeDescriptor) -> Value {
    match (&typ.parts) {
        TypeDescriptorParts::BuiltIn(name) => into_builtin_value_type(&name, &typ.constr),
        TypeDescriptorParts::Custom(parts) => into_custom_value_type(&parts, &typ.constr)
    }
}
fn into_builtin_value_type(_name : &str, _constr : &HashMap<String, Literal>) -> Value {
    todo!();
}
fn into_custom_value_type(_parts : &Vec<String>, _constr : &HashMap<String, Literal>) -> Value {
    todo!();
}
