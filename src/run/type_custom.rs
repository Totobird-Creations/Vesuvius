use crate::run::notes::{
    WarnType,
    ErrorType
};

use num_bigint::{
    BigInt,
    BigUint
};


pub trait TryOps<Other> {
    type Output;
    fn try_eq(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>;
    fn try_ne(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return self.try_eq(other).map(|x|!x);
    }
    fn try_gt(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>;
    fn try_le(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return self.try_gt(other).map(|x|!x);
    }
    fn try_lt(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>;
    fn try_ge(&self, other : &Other) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return self.try_lt(other).map(|x|!x);
    }
    fn try_add(&self, other : &Other) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Err((Vec::new(), vec![ErrorType::InvalidTypeReceived]));
    }
    fn try_sub(&self, other : &Other) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Err((Vec::new(), vec![ErrorType::InvalidTypeReceived]));
    }
    fn try_mul(&self, other : &Other) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Err((Vec::new(), vec![ErrorType::InvalidTypeReceived]));
    }
    fn try_div(&self, other : &Other) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Err((Vec::new(), vec![ErrorType::InvalidTypeReceived]));
    }
}


impl TryOps<Self> for bool {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
}


impl TryOps<Self> for i8 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for i16 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for i32 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for i64 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for i128 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for BigInt {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self + other);
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self - other);
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self * other);
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self / other);
    }
}


impl TryOps<Self> for u8 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for u16 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for u32 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for u64 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for u128 {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_add(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_sub(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_mul(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if let Some(value) = self.checked_div(*other) {
            Ok(value)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
}
impl TryOps<Self> for BigUint {
    type Output = Self;
    fn try_eq(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self == other);
    }
    fn try_gt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self > other);
    }
    fn try_lt(&self, other : &Self) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self < other);
    }
    fn try_add(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self + other);
    }
    fn try_sub(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return if (self >= other) {
            Ok(self - other)
        } else {Err((Vec::new(), vec![ErrorType::Bound_Broken]))}
    }
    fn try_mul(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self * other);
    }
    fn try_div(&self, other : &Self) -> Result<Self::Output, (Vec<WarnType>, Vec<ErrorType>)> {
        return Ok(self / other);
    }
}
