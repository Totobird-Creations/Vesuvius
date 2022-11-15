pub(crate) mod types;

use crate::{
    parse::node::*,
    scope::Scope,
    check::types::{
        Value,
        ValueType
    },
    notes::{
        push_warn,
        push_error
    }
};



impl Program {

    pub(crate) fn register_decls(&self, scope : &mut Scope) {
        push_warn!(InternalWarning, Always, {
            None => {"Todo : Declaration Headers"}
        });
        self.decls.iter().for_each(|decl| decl.register(scope));
    }

    pub(crate) fn expand_types(&self, _scope : &mut Scope) {
        push_warn!(InternalWarning, Always, {
            None => {"Todo : Expand Types"}
        })
    }

    pub(crate) fn check_contents(&self, scope : &mut Scope) {
        self.decls.iter().for_each(|decl| decl.check_contents(scope));
    }

}



impl Declaration {


    fn register(&self, scope : &mut Scope) {
        use DeclarationType::*;
        match (&self.decl) {

            Module(parts, _) => {
                scope.init_symbol(
                    parts[parts.len() - 1].clone(),
                    Value::new(
                        ValueType::ModuleAccess(parts.clone()),
                        self.range.clone()
                    )
                );
            },

            Function(name, _, args, ret, block) => {
                scope.init_symbol(
                    name.clone(),
                    Value::new(
                        ValueType::Function(name.clone(), args.clone(), ret.clone(), block.clone()),
                        self.range.clone()
                    )
                );
            }

        }
    }


    fn check_contents(&self, scope : &mut Scope) {
        use DeclarationType::*;
        match (&self.decl) {

            Module(_, _) => {},

            Function(name, _, _, _, block) => {
                let _ = block.check_contents(scope, name.clone());
            }

        }
    }


}



impl Statement {

    pub fn check_contents(&self, _scope : &mut Scope) -> Value {
        push_error!(InternalError, Always, {
            None => {"Todo : Check Contents"}
        });
        return Value::new(ValueType::Failed, self.range.clone());
    }

}



impl Block {

    pub fn check_contents(&self, scope : &mut Scope, name : String) -> Value {
        let mut subscope = scope.enter(name);
        let mut ret      = ValueType::Void;
        for stmt in &self.stmts {
            ret = stmt.check_contents(&mut subscope).value();
        }
        return Value::new(if (self.retlast) {
            ret
        } else {ValueType::Void}, self.range.clone());
    }

}
