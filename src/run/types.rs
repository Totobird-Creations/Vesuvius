use std::cmp::Ordering;
use std::str::FromStr;

use num_bigint::{
    BigInt,
    ToBigInt
};
use num_bigfloat::BigFloat;

use crate::parser::node::*;


#[derive(Debug, Clone)]
pub struct ValConstr<T : PartialEq>(Vec<T>);
impl<T : PartialEq> ValConstr<T> {
    pub fn equals(&self, other : &ValConstr<T>) -> Value {
        let mut t = false;
        let mut f = false;
        for sval in self.0 {
            for oval in other.0 {
                if (sval == oval) {t = true;}
                if (sval != oval) {f = true;}
                if (t && f) {break;}
            }
            if (t && f) {break;}
        }
        let v = Vec::new();
        if (t) {v.push(true);}
        if (f) {v.push(false);}
        return Value::Bool(ValConstr(v));
    }
}

#[derive(Debug, Clone)]
pub struct ValConstrOrd<T : PartialEq + PartialOrd>(Vec<ValConstrRange<T>>);
impl<T : PartialEq + PartialOrd> ValConstrOrd<T> {
    pub fn equals(&self, other : &ValConstrOrd<T>) -> Value {
        let mut t = false;
        let mut f = false;
        for sval in self.0 {
            for oval in other.0 {
                sval.equals(&oval, &mut t, &mut f);
                if (t && f) {break;}
            }
            if (t && f) {break;}
        }
        let v = Vec::new();
        if (t) {v.push(true);}
        if (f) {v.push(false);}
        return Value::Bool(ValConstr(v));
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
        return match ((self, other)) {
            (Self::Int   (l) , Self::Int   (r) ) => {l.equals(&r)},
            (Self::Float (l) , Self::Float (r) ) => {l.equals(&r)},
            (Self::Bool  (l) , Self::Bool  (r) ) => {l.equals(&r)},
            _ => {Value::Bool(ValConstr(vec![false]))}
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
