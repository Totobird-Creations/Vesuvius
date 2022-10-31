use std::collections::HashMap;

use crate::parser::node::*;


use num_bigint::{
    BigInt,
    BigUint
};
use num_bigfloat::{
    BigFloat
};



#[derive(Clone)]
pub struct Context {
    pub entry   : Option<Vec<String>>,
    pub module  : Vec<String>,
    pub symbols : HashMap<String, (DeclarationVisibility, Value)>
}


#[derive(Debug, Clone)]
pub enum ValConstr<T : PartialEq + PartialOrd> {
    AnyOf    (Box<Vec<ValConstr<T>>>), // List of possible constraints
    InRange  (T, T),                   // Min (inclusive), Max (exclusive)
    IsValue  (T)                       // Value
}
impl<T : PartialEq + PartialOrd> ValConstr<T> {

    pub fn test(&self, value : &T) -> bool {
        return match (self) {

            ValConstr::AnyOf(vs) => vs.iter().any(|v| v.test(value)),

            ValConstr::InRange(min, max) => value >= min && value < max,

            ValConstr::IsValue(v) => value == v

        }
    }

}


#[derive(Debug, Clone)]
pub enum Value {
    Void,

    FuncType(Vec<(String, Type)>, Option<Type>, Block),

    Int(ValConstr<BigInt>),
    Uint(ValConstr<BigUint>),

    Float(BigFloat),
    UFloat(BigFloat)

}



impl Declaration {
    pub fn pre_verify(&self, context : &mut Context) {
        match (&self.decl) {

            DeclarationType::Function(name, args, ret, block) => {
                for header in &self.headers {
                    if let DeclarationHeader::Entry = header {
                        if matches!(context.entry, Some(_)) {
                            // TODO : ADD PROPER ERROR
                            panic!("Duplicate `{}` definition.", header.format(0));
                        } else if ! matches!(self.vis, DeclarationVisibility::Public) {
                            // TODO : ADD PROPER ERROR
                            panic!("`{}` functions must be `{}`.", header.format(0), DeclarationVisibility::Public.format(0));
                        } else {
                            let mut loc = context.module.clone();
                            loc.push(name.clone());
                            context.entry = Some(loc);
                        }
                    } else {
                        // TODO : ADD PROPER ERROR
                        panic!("Header `{}` can not be used on functions.", header.format(0));
                    }
                }
            }

        }

        self.decl.pre_verify(self.vis, context)
    }
}

impl DeclarationType {
    pub fn pre_verify(&self, vis : DeclarationVisibility, context : &mut Context) {
        match (self) {

            Self::Function(name, args, ret, block) => {
                context.symbols.insert(name.clone(), (vis, Value::FuncType(
                    args.clone(), ret.clone(), block.clone()
                )));
            }

        }
    }
}
