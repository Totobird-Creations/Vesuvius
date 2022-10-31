use num_bigint::{
    BigInt,
    BigUint
};
use num_bigfloat::{
    BigFloat
};


pub enum Value {
    Void,

    Int(BigInt),
    Uint(BigUint),

    Float(BigFloat),
    UFloat(BigFloat)

}
