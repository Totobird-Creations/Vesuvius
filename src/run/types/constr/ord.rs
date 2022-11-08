use std::collections::HashMap;

use crate::run::{
    notes::{
        WarnType,
        ErrorType
    },
    types::{
        constr::{
            ValConstrState,
            TestResponse
        },
        traits::TryOps,
        op_bool_fn
    }
};


#[derive(Debug, Clone)]
pub struct ValConstrOrd<T : TryOps<T> + Clone>(pub ValConstrState<ValConstrRange<T>>);
impl<T : TryOps<T> + Clone> ValConstrOrd<T> {

    pub fn combine(&self, other : &Self) -> Self {
        return ValConstrOrd(self.0.combine(&other.0));
    }

    pub fn _op_bool_base<F>(&self,
        other  : &T,
        op     : F,
        tf     : &mut (bool, bool),
        warns  : &mut HashMap<WarnType, (bool, bool)>,
        errors : &mut HashMap<ErrorType, (bool, bool)>
    )
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        compile_error!("TODO")
    }

    pub fn op_bool<F>(&self, other : &T, op : F)
        -> TestResponse
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        compile_error!("TODO")
    }

    pub fn op_bool_other<F>(&self, value : &Self, op : F)
        -> TestResponse
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        compile_error!("TODO")
    }

    op_bool_fn!{eq}
    op_bool_fn!{ne}

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
