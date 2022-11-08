pub mod traits;
pub mod constr;

use num_bigint::{
    BigInt,
    BigUint
};
use num_bigfloat::BigFloat;
use paste::paste;

use crate::{
    parse::node::*,
    run::types::{
        constr::{
            eq::ValConstr,
            ord::ValConstrOrd,
            TestResponse
        }
    }
};


#[derive(Debug, Clone)]
pub struct Value {
    pub value : ValueType,
    pub range : Range
}
#[derive(Debug, Clone)]


pub enum ValueType {
    Unknown,

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
    UintBig (ValConstrOrd<BigUint>),

    Float32  (ValConstrOrd<f32>),
    Float64  (ValConstrOrd<f64>),
    FloatBig (ValConstrOrd<BigFloat>)

}


impl Value {

    pub fn bool_from(value : TestResponse, range : Range) -> Value {
        return Value {
            value : ValueType::Bool(ValConstr(
                match (value) {
                    TestResponse::Always    => constr::ValConstrState::Some(vec![true]),
                    TestResponse::Sometimes => constr::ValConstrState::Some(vec![true, false]),
                    TestResponse::Never     => constr::ValConstrState::Some(vec![false]),
                    TestResponse::Failed    => constr::ValConstrState::Failed
                }
            )),
            range
        };
    }

    pub fn type_def(&self) -> String {
        return match (&self.value) {
            ValueType::Unknown => String::from("<?>"),

            ValueType::Void => String::from("void"),

            ValueType::Function(args, ret, _) => format!("fn({}){}",
                args.iter().map(|(name, typ)| format!("{}: {}", name, typ.type_def())).collect::<Vec<_>>().join(""),
                if let Some(ret) = &**ret {format!(" -> {}", ret.type_def())}
                else {String::new()}
            ),

            ValueType::Bool(_) => String::from("bool"),

            ValueType::Int8   (_)   => String::from("int8"),
            ValueType::Int16  (_) => String::from("int16"),
            ValueType::Int32  (_) => String::from("int32"),
            ValueType::Int64  (_) => String::from("int64"),
            ValueType::Int128 (_) => String::from("int128"),
            ValueType::IntBig (_) => String::from("intbig"),

            ValueType::Uint8   (_) => String::from("uint8"),
            ValueType::Uint16  (_) => String::from("uint16"),
            ValueType::Uint32  (_) => String::from("uint32"),
            ValueType::Uint64  (_) => String::from("uint64"),
            ValueType::Uint128 (_) => String::from("uint128"),
            ValueType::UintBig (_) => String::from("uintbig"),

            ValueType::Float32  (_) => String::from("float32"),
            ValueType::Float64  (_) => String::from("float64"),
            ValueType::FloatBig (_) => String::from("floatbig")

        }
    }

    pub fn matches_type(&self, other : &Value) -> bool {
        return if (matches!(self.value, ValueType::Unknown) || matches!(other.value, ValueType::Unknown)) {
            true
        } else {
            match_lr!{(&self.value, &other.value) {

                ValueType::Void => true,

                ValueType::Function (largs/rargs, lret/rret, _) => {
                    if (largs.len() != rargs.len()) {false}
                    else {
                        let pass = match_lr!{(&**lret, &**rret) {
                            Some(_) => true,
                            None    => true,
                            _       => false
                        }};
                        if (! pass) {false}
                        else {
                            let mut pass = true;
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

                ValueType::Float32  (_) => true,
                ValueType::Float64  (_) => true,
                ValueType::FloatBig (_) => true,

                _ => false

            }}
        };
    }

    pub fn combine(&self, other : &Self) -> Self {
        return Value {
            value : match_lr!{(&self.value, &other.value) {

                ValueType::Void => ValueType::Void,

                ValueType::Bool (l/r) => ValueType::Bool(l.combine(&r)),

                ValueType::Int8   (l/r) => ValueType::Int8   (l.combine(&r)),
                ValueType::Int16  (l/r) => ValueType::Int16  (l.combine(&r)),
                ValueType::Int32  (l/r) => ValueType::Int32  (l.combine(&r)),
                ValueType::Int64  (l/r) => ValueType::Int64  (l.combine(&r)),
                ValueType::Int128 (l/r) => ValueType::Int128 (l.combine(&r)),
                ValueType::IntBig (l/r) => ValueType::IntBig (l.combine(&r)),

                ValueType::Uint8   (l/r) => ValueType::Uint8   (l.combine(&r)),
                ValueType::Uint16  (l/r) => ValueType::Uint16  (l.combine(&r)),
                ValueType::Uint32  (l/r) => ValueType::Uint32  (l.combine(&r)),
                ValueType::Uint64  (l/r) => ValueType::Uint64  (l.combine(&r)),
                ValueType::Uint128 (l/r) => ValueType::Uint128 (l.combine(&r)),
                ValueType::UintBig (l/r) => ValueType::UintBig (l.combine(&r)),

                ValueType::Float32  (l/r) => ValueType::Float32  (l.combine(&r)),
                ValueType::Float64  (l/r) => ValueType::Float64  (l.combine(&r)),
                ValueType::FloatBig (l/r) => ValueType::FloatBig (l.combine(&r)),

                _ => ValueType::Unknown

            }},
            range : Range(self.range.0, other.range.1)
        };
    }

    pub fn eq(&self, other : &Value) -> TestResponse {
        return match_lr!{(&self.value, &other.value) {
            ValueType::Unknown => TestResponse::Failed,

            ValueType::Void => TestResponse::Always,

            ValueType::Bool (l/r) => l.eq_other(&r),

            ValueType::Int8   (l/r) => l.eq_other(&r),
            ValueType::Int16  (l/r) => l.eq_other(&r),
            ValueType::Int32  (l/r) => l.eq_other(&r),
            ValueType::Int64  (l/r) => l.eq_other(&r),
            ValueType::Int128 (l/r) => l.eq_other(&r),
            ValueType::IntBig (l/r) => l.eq_other(&r),

            ValueType::Uint8   (l/r) => l.eq_other(&r),
            ValueType::Uint16  (l/r) => l.eq_other(&r),
            ValueType::Uint32  (l/r) => l.eq_other(&r),
            ValueType::Uint64  (l/r) => l.eq_other(&r),
            ValueType::Uint128 (l/r) => l.eq_other(&r),
            ValueType::UintBig (l/r) => l.eq_other(&r),

            ValueType::Float32  (l/r) => l.eq_other(&r),
            ValueType::Float64  (l/r) => l.eq_other(&r),
            ValueType::FloatBig (l/r) => l.eq_other(&r),

            _ => TestResponse::Failed

        }};
    }

    pub fn ne(&self, other : &Value) -> TestResponse {
        return match_lr!{(&self.value, &other.value) {
            ValueType::Unknown => TestResponse::Failed,

            ValueType::Void => TestResponse::Always,

            ValueType::Bool (l/r) => l.ne_other(&r),

            ValueType::Int8   (l/r) => l.ne_other(&r),
            ValueType::Int16  (l/r) => l.ne_other(&r),
            ValueType::Int32  (l/r) => l.ne_other(&r),
            ValueType::Int64  (l/r) => l.ne_other(&r),
            ValueType::Int128 (l/r) => l.ne_other(&r),
            ValueType::IntBig (l/r) => l.ne_other(&r),

            ValueType::Uint8   (l/r) => l.ne_other(&r),
            ValueType::Uint16  (l/r) => l.ne_other(&r),
            ValueType::Uint32  (l/r) => l.ne_other(&r),
            ValueType::Uint64  (l/r) => l.ne_other(&r),
            ValueType::Uint128 (l/r) => l.ne_other(&r),
            ValueType::UintBig (l/r) => l.ne_other(&r),

            ValueType::Float32  (l/r) => l.ne_other(&r),
            ValueType::Float64  (l/r) => l.ne_other(&r),
            ValueType::FloatBig (l/r) => l.ne_other(&r),

            _ => TestResponse::Failed

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


macro_rules! op_bool_fn {
    {$name:ident} => {$crate::run::types::paste!{
        pub fn $name(&self, value : &T) -> TestResponse {
            self.op_bool(value, |a, b| $crate::run::types::paste!{a.[<try_ $name>](b)})
        }
        pub fn [<$name _other>](&self, other : &Self) -> TestResponse {
            self.op_bool_other(other, |a, b| a.[<try_ $name>](b))
        }
    }}
}
use op_bool_fn;
