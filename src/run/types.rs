use std::cmp::{
    Ordering,
    min,
    max
};
use std::str::FromStr;

use num_bigint::{
    BigInt,
    ToBigInt
};
use num_bigfloat::BigFloat;

use crate::parse::node::*;


pub enum TestResponse {
    Always,    // Value matches every value of the constraint.
    Sometimes, // Value matches some values of the constraint.
    Never,     // Value does not match any value of the constraint.
    Failed     // A previous operation failed, so this test could not be performed.
}

#[derive(Debug, Clone)]
pub struct ValConstr<T : PartialEq + Clone>(pub Vec<T>);
impl<T : PartialEq + Clone> ValConstr<T> {
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

    pub fn equals(&self, other : &ValConstr<T>, range : Range) -> Value {
        if (self.0.len() <= 0) {
            return Value {
                value : ValueType::Bool(ValConstr::failed()),
                range
            };
        }
        let mut t = false;
        let mut f = false;
        for sval in &self.0 {
            for oval in &other.0 {
                if (sval == oval) {t = true;}
                if (sval != oval) {f = true;}
                if (t && f) {break;}
            }
            if (t && f) {break;}
        }
        let mut v = Vec::new();
        if (t) {v.push(true);}
        if (f) {v.push(false);}
        return Value {
            value : ValueType::Bool(ValConstr(v)),
            range
        };
    }

}

#[derive(Debug, Clone)]
pub struct ValConstrOrd<T : PartialEq + PartialOrd + Clone>(pub Vec<ValConstrRange<T>>);
impl<T : PartialEq + PartialOrd + Clone> ValConstrOrd<T> {

    pub fn combine(&self, other : &ValConstrOrd<T>) -> ValConstrOrd<T> {
        let mut joined = self.0.clone();
        joined.append(&mut other.0.clone());
        return ValConstrOrd(joined);
    }

    pub fn equals(&self, other : &ValConstrOrd<T>, range : Range) -> Value {
        let mut t = false;
        let mut f = false;
        for sval in &self.0 {
            for oval in &other.0 {
                sval.equals(&oval, &mut t, &mut f);
                if (t && f) {break;}
            }
            if (t && f) {break;}
        }
        let mut v = Vec::new();
        if (t) {v.push(true);}
        if (f) {v.push(false);}
        return Value {
            value : ValueType::Bool(ValConstr(v)),
            range
        };
    }

}

#[derive(Debug, Clone)]
pub enum ValConstrRange<T : PartialEq + PartialOrd> {
    Exact(T),
    MinInMaxIn(T, T)
}
impl<T : PartialEq + PartialOrd> ValConstrRange<T> {
    pub fn equals(&self, other : &ValConstrRange<T>, t : &mut bool, f : &mut bool) {
        match (self) {

            Self::Exact(a) => {
                match (other) {

                    Self::Exact(b) => {
                        if (a == b) {*t = true;}
                        else {*f = true;}
                    },

                    Self::MinInMaxIn(bi, ba) => {
                        if (bi == a && ba == a) {*t = true;}
                        else if (bi <= a && ba >= a) {*t = true; *f = true;}
                        else {*f = true;}
                    }
                }
            },

            Self::MinInMaxIn(ai, aa) => {
                match (other) {

                    Self::Exact(b) => {
                        if (ai == b && aa == b) {*t = true;}
                        else if (ai <= b && aa >= b) {*t = true; *f = true;}
                        else {*f = true;}
                    },

                    Self::MinInMaxIn(bi, ba) => {
                        if (ai == bi && aa == ba) {*t = true;}
                        else if (
                               ai <= bi && aa >= bi
                            || ai <= ba && aa >= ba
                            || bi <= ai && ba >= ai
                            || bi <= aa && ba >= aa
                        ) {*t = true; *f = true;}
                        else {*f = true;}
                    }

                }
            }

        }
    }
}


#[derive(Debug, Clone)]
pub struct Value {
    pub value : ValueType,
    pub range : Range
}
#[derive(Debug, Clone)]
pub enum ValueType {
    Void,

    Function(Box<Vec<(String, Value)>>, Box<Option<Value>>, Block),

    Int(ValConstrOrd<ValuePossiblyBigInt>),
    Float(ValConstrOrd<ValuePossiblyBigFloat>),

    Bool(ValConstr<bool>)

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

            ValueType::Int(_)   => String::from("int"),

            ValueType::Float(_) => String::from("float"),

            ValueType::Bool(_)  => String::from("bool")

        }
    }

    pub fn matches_type(&self, other : &Value) -> bool {
        return
            if      (matches!(self.value, ValueType::Void      ) && matches!(other.value, ValueType::Void      )) {true}
            else if (matches!(self.value, ValueType::Int   (_) ) && matches!(other.value, ValueType::Int   (_) )) {true}
            else if (matches!(self.value, ValueType::Float (_) ) && matches!(other.value, ValueType::Float (_) )) {true}
            else if (matches!(self.value, ValueType::Bool  (_) ) && matches!(other.value, ValueType::Bool  (_) )) {true}
            else {false}
    }

    pub fn combine(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Void      , ValueType::Void      ) => {Value {value : ValueType::Void                  , range}},
            (ValueType::Int   (l) , ValueType::Int   (r) ) => {Value {value : ValueType::Int   (l.combine(&r)) , range}},
            (ValueType::Float (l) , ValueType::Float (r) ) => {Value {value : ValueType::Float (l.combine(&r)) , range}},
            (ValueType::Bool  (l) , ValueType::Bool  (r) ) => {Value {value : ValueType::Bool  (l.combine(&r)) , range}},
            _ => {panic!("INTERNAL ERROR")}
        };
    }

    pub fn equals(&self, other : &Value) -> Value {
        let range = Range(min(self.range.0, other.range.0), max(self.range.1, other.range.1));
        return match ((&self.value, &other.value)) {
            (ValueType::Void      , ValueType::Void      ) => {Value {value : ValueType::Bool(ValConstr(vec![true])), range : range}},
            (ValueType::Int   (l) , ValueType::Int   (r) ) => {l.equals(&r, range)},
            (ValueType::Float (l) , ValueType::Float (r) ) => {l.equals(&r, range)},
            (ValueType::Bool  (l) , ValueType::Bool  (r) ) => {l.equals(&r, range)},
            _ => {Value {
                value : ValueType::Bool(ValConstr(vec![false])),
                range
            }}
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
            Self::Small(a) => {match (other) {
                Self::Small (b) => {a == b}
                Self::Big   (b) => {&a.to_bigint().unwrap() == b}
            }},
            Self::Big(a) => {match (other) {
                Self::Small (b) => {a == &b.to_bigint().unwrap()}
                Self::Big   (b) => {a == b}
            }}
        }
    }
}
impl PartialOrd for ValuePossiblyBigInt {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        match (self) {
            Self::Small(a) => {match (other) {
                Self::Small (b) => {a.partial_cmp(b)}
                Self::Big   (b) => {a.to_bigint().unwrap().partial_cmp(b)}
            }},
            Self::Big(a) => {match (other) {
                Self::Small (b) => {a.partial_cmp(&b.to_bigint().unwrap())}
                Self::Big   (b) => {a.partial_cmp(b)}
            }}
        }
    }
}
impl From<&String> for ValuePossiblyBigInt {
    fn from(value : &String) -> Self {
        let res = value.parse::<i64>();
        return if let Ok(res) = res {
            Self::Small(res)
        } else {
            Self::Big(BigInt::from_str(value).unwrap())
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
            Self::Small(a) => {match (other) {
                Self::Small (b) => {a == b}
                Self::Big   (b) => {&BigFloat::from_f64(*a) == b}
            }},
            Self::Big(a) => {match (other) {
                Self::Small (b) => {a == &BigFloat::from_f64(*b)}
                Self::Big   (b) => {a == b}
            }}
        }
    }
}
impl PartialOrd for ValuePossiblyBigFloat {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        match (self) {
            Self::Small(a) => {match (other) {
                Self::Small (b) => {a.partial_cmp(b)}
                Self::Big   (b) => {BigFloat::from_f64(*a).partial_cmp(b)}
            }},
            Self::Big(a) => {match (other) {
                Self::Small (b) => {a.partial_cmp(&BigFloat::from_f64(*b))}
                Self::Big   (b) => {a.partial_cmp(b)}
            }}
        }
    }
}
impl From<&String> for ValuePossiblyBigFloat {
    fn from(value : &String) -> Self {
        let res = value.parse::<f64>();
        return if let Ok(res) = res {
            Self::Small(res)
        } else {
            Self::Big(BigFloat::parse(value).unwrap())
        }
    }
}
