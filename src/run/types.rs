use std::cmp::{
    min,
    max
};
use std::collections::HashMap;

use num_bigint::{
    BigInt,
    BigUint
};

use crate::parse::node::*;
use crate::run::type_custom::*;
use crate::run::notes::{
    WarnType,
    ErrorType
};


pub enum TestResponse {
    Always,    // Value matches every value of the constraint.
    Sometimes, // Value matches some values of the constraint.
    Never,     // Value does not match any value of the constraint.
    Failed     // A previous operation failed, so this test could not be performed.
}

#[derive(Debug, Clone)]
pub struct ValConstr<T : TryOps<T> + Clone>(pub ValConstrState<T>);
impl<T : TryOps<T> + Clone> ValConstr<T> {

    pub fn failed() -> Self {
        return ValConstr(ValConstrState::Failed);
    }

    pub fn when_eq_to(&self, other : &T) -> TestResponse {
        return match (self.0) {
            ValConstrState::Failed        => TestResponse::Failed,
            ValConstrState::Some(vals)    => compile_error!("TODO"),
            ValConstrState::Unconstrained => TestResponse::Sometimes
        };
    }

    pub fn op_bool<F>(&self, other : &Self, op : F)
        -> TestResponse
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        return match ((&self.0, &other.0)) {
            (ValConstrState::Failed, _)                        => TestResponse::Failed,
            (_, ValConstrState::Failed)                        => TestResponse::Failed,
            (ValConstrState::Unconstrained, _)                 => TestResponse::Sometimes,
            (_, ValConstrState::Unconstrained)                 => TestResponse::Sometimes,
            (ValConstrState::Some(ls), ValConstrState::Some(rs)) => {
                let mut tf     = (false, false);
                let mut warns  = HashMap::new();
                let mut errors = HashMap::new();
                for l in ls {
                    for r in rs {
                        match (op(l, r)) {
                            Ok(val) => {
                                if (val) {tf.0 = true;}
                                else {tf.1 = true;}
                            },
                            Err((warn, error)) => {
                                for typ in warn {
                                    if (! warns.contains_key(&typ)) {
                                        warns.insert(typ, (false, false));
                                    }
                                }
                                for typ in warns.keys() {
                                    let (t, f) = warns.get_mut(typ).unwrap();
                                    if (warn.contains(typ)) {*t = true;}
                                    else {*f = true;}
                                }
                                for typ in error {
                                    if (! errors.contains_key(&typ)) {
                                        errors.insert(typ, (false, false));
                                    }
                                }
                                for typ in errors.keys() {
                                    let (t, f) = errors.get_mut(typ).unwrap();
                                    if (error.contains(typ)) {*t = true;}
                                    else {*f = true;}
                                }
                            }
                        }
                    }
                }
                return match (tf) {
                    (false, false) => TestResponse::Failed,
                    (true, false)  => TestResponse::Always,
                    (false, true)  => TestResponse::Never,
                    (true, true)   => TestResponse::Sometimes
                };
            }
        };
    }

    pub fn eq(&self, other : &Self) -> TestResponse {
        self.op_bool(other, |a, b| a.try_eq(b))
    }
    pub fn ne(&self, other : &Self) -> TestResponse {
        self.op_bool(other, |a, b| a.try_ne(b))
    }

}


#[derive(Debug, Clone)]
pub struct ValConstrOrd<T : TryOps<T> + Clone>(pub ValConstrState<ValConstrRange<T>>);
impl<T : TryOps<T> + Clone> ValConstrOrd<T> {

    pub fn failed() -> Self {
        return ValConstrOrd(ValConstrState::Failed);
    }

}


#[derive(Debug, Clone)]
pub enum ValConstrRange<T : TryOps<T> + Clone> {
    Exact(T),
    MinInMaxIn(T, T)
}
impl<T : TryOps<T> + Clone> ValConstrRange<T> {

    pub fn test(&self, other : &T) -> bool {
        return match (self) {
            ValConstrRange::Exact      (val)      => other.try_eq(val).unwrap_or(false),
            ValConstrRange::MinInMaxIn (min, max) => other.try_ge(min).unwrap_or(false) && other.try_le(max).unwrap_or(false)
        };
    }

}


#[derive(Debug, Clone)]
pub enum ValConstrState<T> {
    Failed,       // Previous operation failed. Type is known, but possible values are not.
    Some(Vec<T>), // A list of possible values.
    Unconstrained // Any value will pass.
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

    Bool(ValConstr<bool>),

    Int8   (ValConstrOrd<i8>),
    Int16  (ValConstrOrd<i16>),
    Int32  (ValConstrOrd<i32>),
    Int64  (ValConstrOrd<i64>),
    Int128 (ValConstrOrd<i128>),
    IntBig (ValConstrOrd<BigInt>),

    Uint8   (ValConstrOrd<u8>),
    Uint16  (ValConstrOrd<u16>),
    Uint32  (ValConstrOrd<u32>),
    Uint64  (ValConstrOrd<u64>),
    Uint128 (ValConstrOrd<u128>),
    UintBig (ValConstrOrd<BigUint>)

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

            ValueType::Bool(_) => String::from("bool"),

            ValueType::Int8(_) => String::from("int8"),
            ValueType::Int16(_) => String::from("int16"),
            ValueType::Int32(_) => String::from("int32"),
            ValueType::Int64(_) => String::from("int64"),
            ValueType::Int128(_) => String::from("int128"),
            ValueType::IntBig(_) => String::from("intbig"),

            ValueType::Uint8(_) => String::from("uint8"),
            ValueType::Uint16(_) => String::from("uint16"),
            ValueType::Uint32(_) => String::from("uint32"),
            ValueType::Uint64(_) => String::from("uint64"),
            ValueType::Uint128(_) => String::from("uint128"),
            ValueType::UintBig(_) => String::from("uintbig")

        }
    }

    pub fn matches_type(&self, other : &Value) -> bool {
        return match_lr!{(self.value, other.value) {

            ValueType::Void => true,

            ValueType::Function (largs/rargs, lret/rret, _) => {
                if (largs.len() != rargs.len()) {false}
                else {
                    let pass = match_lr!{(*lret, *rret) {
                        Some(lret/rret) => true,
                        None            => true,
                        _               => false
                    }};
                    if (! pass) {false}
                    else {
                        let pass = true;
                        for i in 0..largs.len() {
                            pass = largs[i].1.matches_type(&rargs[i].1);
                            if (! pass) {break;}
                        }
                        pass
                    }
                }
            },

            ValueType::Bool (_) => true,

            ValueType::Int8   (_) => true,
            ValueType::Int16  (_) => true,
            ValueType::Int32  (_) => true,
            ValueType::Int64  (_) => true,
            ValueType::Int128 (_) => true,
            ValueType::IntBig (_) => true,

            ValueType::Uint8   (_) => true,
            ValueType::Uint16  (_) => true,
            ValueType::Uint32  (_) => true,
            ValueType::Uint64  (_) => true,
            ValueType::Uint128 (_) => true,
            ValueType::UintBig (_) => true,

            _ => false

        }};
    }

}



macro_rules! match_lr {
    {($left:expr, $right:expr) {
        $($($variant:ident)::+ $(($($arg:tt $(/ $arg2:ident)?),*))? => $expr:expr),*
        , _ => $fbexpr:expr
    }} => {match (($left, $right)) {
        $(
            (
                $($variant)::+$(($($crate::run::types::_match_lr_a!(l => $arg $(/ $arg2)?)),*))?,
                $($variant)::+$(($($crate::run::types::_match_lr_a!(r => $arg $(/ $arg2)?)),*))?
            )
            => {$expr}
        ),*
        , _ => $fbexpr
    }}
}
use match_lr;

macro_rules! _match_lr_a {
    (l => _) => {_};
    (r => _) => {_};
    (l => $lname:ident/$_:ident) => {$lname};
    (r => $_:ident/$rname:ident) => {$rname};
}
use _match_lr_a;
