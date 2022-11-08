use std::collections::HashMap;

use crate::verify::{
    notes::{
        WarnType,
        ErrorType,
        push_error
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
        _other  : &ValConstrRange<T>,
        _op     : F,
        _tf     : &mut (bool, bool),
        _warns  : &mut HashMap<WarnType, (bool, bool)>,
        _errors : &mut HashMap<ErrorType, (bool, bool)>
    )
        where F : Fn(&ValConstrRange<T>, &ValConstrRange<T>) -> Result<TestResponse, (Vec<WarnType>, Vec<ErrorType>)>
    {
        // TODO
        todo!();
    }

    pub fn op_bool<F>(&self, _value : &T, _op : F)
        -> TestResponse
        where F : Fn(&ValConstrRange<T>, &ValConstrRange<T>) -> Result<TestResponse, (Vec<WarnType>, Vec<ErrorType>)>
    {
        push_error!(InternalTodo, Always);
        return TestResponse::Failed;
    }

    pub fn op_bool_other<F>(&self, _other : &Self, _op : F)
        -> TestResponse
        where F : Fn(&ValConstrRange<T>, &ValConstrRange<T>) -> Result<TestResponse, (Vec<WarnType>, Vec<ErrorType>)>
    {
        push_error!(InternalTodo, Always);
        return TestResponse::Failed;
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

    pub fn try_eq(&self, _other : &Self) -> Result<TestResponse, (Vec<WarnType>, Vec<ErrorType>)> {
        todo!();
    }

    pub fn try_ne(&self, _other : &Self) -> Result<TestResponse, (Vec<WarnType>, Vec<ErrorType>)> {
        todo!();
    }

}
