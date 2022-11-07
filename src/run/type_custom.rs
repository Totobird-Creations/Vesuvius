pub trait TryBoolOps<Other> {
    type Output;
    fn try_eq(&self, other : Other) -> Option<bool>;
    fn try_ne(&self, other : Other) -> Option<bool> {
        return self.try_eq(other).map(|x|!x);
    }
}
pub trait TryOrdOps<Other> {
    type Output;
    fn try_add(&self, other : Other) -> Option<Self::Output>;
    fn try_sub(&self, other : Other) -> Option<Self::Output>;
    fn try_div(&self, other : Other) -> Option<Self::Output>;
    fn try_mul(&self, other : Other) -> Option<Self::Output>;
}
pub trait TryOps<Other> : TryBoolOps<Other> + TryOrdOps<Other> {}


// -1, -0, 0, 1
#[derive(Debug, Copy, Clone)]
pub struct i2(
    bool, // Sign
    bool  // Value
);
impl i2 {
    pub const POS  : i2 = i2(true, true);
    pub const ZERO : i2 = i2(true, false);
    pub const NEG  : i2 = i2(false, true);
}

// -1, -0, 0, 1
#[derive(Debug, Copy, Clone)]
pub struct i4(
    bool, // Sign
    bool, // Value
    bool,
    bool
);
impl i4 {

}
