use std::cmp::{
    min,
    max
};

use num_bigint::{
    BigInt,
    BigUint
};

use crate::parse::node::*;
use crate::run::type_custom::*;
use crate::run::notes::push_error;


pub enum TestResponse {
    Always,    // Value matches every value of the constraint.
    Sometimes, // Value matches some values of the constraint.
    Never,     // Value does not match any value of the constraint.
    Failed     // A previous operation failed, so this test could not be performed.
}

#[derive(Debug, Clone)]
pub struct ValConstr<T : TryBoolOps<Self>>(pub Vec<T>);
impl<T : TryBoolOps<Self>> ValConstr<T> {
    pub fn failed() -> ValConstr<T> {
        return ValConstr(Vec::new());
    }

    pub fn combine(&self, other : &ValConstr<T>) -> ValConstr<T> {
        let mut joined = self.0.clone();
        joined.append(&mut other.0.clone());
        return ValConstr(joined);
    }

    pub fn test(&self, value : &T) -> TestResponse {
        if (self.0.len() <= 0) {
            return TestResponse::Failed;
        }
        let t = self.0.contains(value);
        let f = self.0.iter().any(|v| v != value);
        return match ((t, f)) {
            (true, false) => TestResponse::Always,
            (false, true) => TestResponse::Never,
            (true, true)  => TestResponse::Sometimes,
            _ => panic!("INTERNAL ERROR")
        };
    }

}

#[derive(Debug)]
pub struct ValConstrOrd<T : TryOps<Self>>(pub ValConstrState<ValConstrRange<T>>);
impl<T : TryOps<Self>> ValConstrOrd<T> {

    pub fn combine(&self, other : &ValConstrOrd<T>) -> ValConstrOrd<T> {
        let mut joined = self.0.clone();
        joined.append(&mut other.0.clone());
        return ValConstrOrd(joined);
    }

}

#[derive(Debug)]
pub enum ValConstrRange<T : PartialEq + PartialOrd> {
    Exact(T),
    MinInMaxIn(T, T)
}
impl<T : PartialEq + PartialOrd> ValConstrRange<T> {

}

#[derive(Debug)]
pub enum ValConstrState<T> {
    Failed,       // Previous operation failed. Type is known, but possible values are not.
    Some(Vec<T>), // A list of possible values.
    Unconstrained // Any value will pass.
}


#[derive(Debug)]
pub struct Value {
    pub value : ValueType,
    pub range : Range
}
#[derive(Debug)]
pub enum ValueType {
    Void,

    Function(Box<Vec<(String, Value)>>, Box<Option<Value>>, Block),

    Int2(ValConstrOrd<i2>),
    Int4(ValConstrOrd<i4>),
    Int8(ValConstrOrd<i8>),
    Int16(ValConstrOrd<i16>),
    Int32(ValConstrOrd<i32>),
    Int64(ValConstrOrd<i64>),
    Int128(ValConstrOrd<i128>),
    IntBig(ValConstrOrd<BigInt>),

    Bool(ValConstr<bool>),
    Uint2(ValConstrOrd<u2>),
    Uint4(ValConstrOrd<u4>),
    Uint8(ValConstrOrd<u8>),
    Uint16(ValConstrOrd<u16>),
    Uint32(ValConstrOrd<u32>),
    Uint64(ValConstrOrd<u64>),
    Uint128(ValConstrOrd<u128>),
    UintBig(ValConstrOrd<BigUint>)

}

impl Value {

    pub fn type_def(&self) -> String {
        return match (&self.value) {

            ValueType::Void                   => String::from("void"),

            ValueType::Function(args, ret, _) => format!("fn({}){}",
                args.iter().map(|(name, typ)| format!("{}: {}", name, typ.type_def())).collect::<Vec<_>>().join(""),
                if let Some(ret) = &**ret {format!(" -> {}", ret.type_def())}
                else {String::new()}
            ),

            ValueType::Int64(_)   => String::from("int64"),

            ValueType::Float64(_) => String::from("float64"),

            ValueType::Bool(_)  => String::from("bool")

        }
    }

    pub fn matches_type(&self, other : &Value) -> bool {
        return
            if      (matches!(self.value, ValueType::Void        ) && matches!(other.value, ValueType::Void        )) {true}
            else if (matches!(self.value, ValueType::Bool    (_) ) && matches!(other.value, ValueType::Bool    (_) )) {true}
            else if (matches!(self.value, ValueType::Int64   (_) ) && matches!(other.value, ValueType::Int64   (_) )) {true}
            else if (matches!(self.value, ValueType::Float64 (_) ) && matches!(other.value, ValueType::Float64 (_) )) {true}
            else {false}
    }

    pub fn combine(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Void        , ValueType::Void        ) => {Value {value : ValueType::Void                    , range}},
            (ValueType::Bool    (l) , ValueType::Bool    (r) ) => {Value {value : ValueType::Bool    (l.combine(&r)) , range}},
            (ValueType::Int64   (l) , ValueType::Int64   (r) ) => {Value {value : ValueType::Int64   (l.combine(&r)) , range}},
            (ValueType::Float64 (l) , ValueType::Float64 (r) ) => {Value {value : ValueType::Float64 (l.combine(&r)) , range}},
            _ => {panic!("INTERNAL ERROR")}
        };
    }

    pub fn equals(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Void        , ValueType::Void        ) => {Value {value : ValueType::Bool(ValConstr(vec![true])), range}},
            (ValueType::Bool    (l) , ValueType::Bool    (r) ) => {l.equals(&r, range)},
            (ValueType::Int64   (l) , ValueType::Int64   (r) ) => {l.equals(&r, range)},
            (ValueType::Float64 (l) , ValueType::Float64 (r) ) => {l.equals(&r, range)},
            _ => {Value {
                value : ValueType::Bool(ValConstr(vec![false])),
                range
            }}
        };
    }

    pub fn not_equals(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Void        , ValueType::Void        ) => {Value {value : ValueType::Bool(ValConstr(vec![true])), range}},
            (ValueType::Bool    (l) , ValueType::Bool    (r) ) => {l.not_equals(&r, range)},
            (ValueType::Int64   (l) , ValueType::Int64   (r) ) => {l.not_equals(&r, range)},
            (ValueType::Float64 (l) , ValueType::Float64 (r) ) => {l.not_equals(&r, range)},
            _ => {Value {
                value : ValueType::Bool(ValConstr(vec![true])),
                range
            }}
        };
    }

    pub fn division(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Int64   (l) , ValueType::Int64   (r) ) => {l.division(&r, range)},
            (ValueType::Float64 (l) , ValueType::Float64 (r) ) => {l.division(&r, range)},
            _ => {push_error!(InvalidTypeReceived, Always, {
                range => {"Both sides must be `int` or `float`."}
            })}
        };
    }

}
