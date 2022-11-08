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
pub struct ValConstr<T : TryOps<T> + Clone>(pub ValConstrState<T>);
impl<T : TryOps<T> + Clone> ValConstr<T> {

    pub fn combine(&self, other : &Self) -> Self {
        return ValConstr(self.0.combine(&other.0));
    }

    pub fn _op_bool_base<F>(&self,
        other  : &T,
        op     : &F,
        tf     : &mut (bool, bool),
        warns  : &mut HashMap<WarnType, (bool, bool)>,
        errors : &mut HashMap<ErrorType, (bool, bool)>
    )
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        return match (&self.0) {
            ValConstrState::Failed        => {*tf = (false, false)},
            ValConstrState::Unconstrained => (*tf = (true, true)),
            ValConstrState::Some(values)  => {
                for value in values {
                    match (op(value, other)) {
                        Ok(val) => {
                            if (val) {tf.0 = true;}
                            else {tf.1 = true;}
                        },
                        Err((warn, error)) => {
                            for typ in &warn {
                                if (! warns.contains_key(&typ)) {
                                    warns.insert(typ.clone(), (false, false));
                                }
                            }
                            for (typ, (t, f)) in &mut *warns {
                                if (warn.contains(typ)) {*t = true;}
                                else {*f = true;}
                            }
                            for typ in &error {
                                if (! errors.contains_key(&typ)) {
                                    errors.insert(typ.clone(), (false, false));
                                }
                            }
                            for (typ, (t, f)) in &mut *errors {
                                if (error.contains(typ)) {*t = true;}
                                else {*f = true;}
                            }
                        }
                    }
                }
            }
        };
    }

    pub fn op_bool<F>(&self, value : &T, op : F)
        -> TestResponse
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        let mut tf     = (false, false);
        let mut warns  = HashMap::new();
        let mut errors = HashMap::new();
        self._op_bool_base(value, &op, &mut tf, &mut warns, &mut errors);
        return match (tf) {
            (false, false) => TestResponse::Failed,
            (true, false)  => TestResponse::Always,
            (false, true)  => TestResponse::Never,
            (true, true)   => TestResponse::Sometimes
        };
    }

    pub fn op_bool_other<F>(&self, other : &Self, op : F)
        -> TestResponse
        where F : Fn(&T, &T) -> Result<bool, (Vec<WarnType>, Vec<ErrorType>)>
    {
        return match (&other.0) {
            ValConstrState::Failed        => TestResponse::Failed,
            ValConstrState::Unconstrained => TestResponse::Sometimes,
            ValConstrState::Some(others)  => {
                let mut tf     = (false, false);
                let mut warns  = HashMap::new();
                let mut errors = HashMap::new();
                for other in others {
                    self._op_bool_base(other, &op, &mut tf, &mut warns, &mut errors);
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

    op_bool_fn!{eq}
    op_bool_fn!{ne}

}
