use crate::parser::node::*;



pub trait ValConstrT : Eq + Ord {}

pub enum ValConstr<T : ValConstrT> {
    AnyOf    (Box<Vec<ValConstr<T>>>), // List of possible constraints
    InRange  (T, T),                   // Min (inclusive), Max (exclusive)
    IsValue  (T),                      // Value
    ButNot (
        Box<ValConstr<T>>,             // Constraints to require
        Box<ValConstr<T>>              // Override above
    )
}
impl<T : ValConstrT> ValConstr<T> {
    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            ValConstr::AnyOf(vs) => vs.iter().any(|v| v.test(value)),

            ValConstr::InRange(min, max) => value >= min && value < max,

            ValConstr::IsValue(v) => value == v,

            ValConstr::ButNot(v, n) => ! n.test(value) && v.test(value)

        }
    }
}



impl Program {
    pub fn verify() {
        todo!();
    }
}
