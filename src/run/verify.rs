use std::collections::HashMap;

use crate::parser::node::*;
use crate::run::types::*;
use crate::run::notes::{
    push_warn,
    push_error
};
use crate::run::scope::*;


impl Program {
    pub fn verify(&self) {
        for decl in &self.decls {
            decl.verify_register();
        }
        for decl in &self.decls {
            decl.verify_contents();
        }
    }
}


impl Declaration {

    // Register that the function exists, so that other objects can reference it.
    fn verify_register(&self) {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                Scope::add_symbol(name, Symbol::from(Value::Function(
                    Box::new(Vec::new()), Box::new(None), block.clone()
                )));
            }

        }
    }

    // Verify the contents of the declaration.
    fn verify_contents(&self) {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                block.verify(Some(name));
            }

        }
    }

}


impl Statement {
    fn verify(&self) -> Value {
        return match (self) {

            Statement::Expression(expr) => {
                expr.verify()
            },

            Statement::InitVar(name, value) => {
                Scope::add_symbol(name, Symbol::from(value.verify()));
                
                return Value::Void;
            }

        }
    }
}


impl Expression {
    fn verify(&self) -> Value {
        return match (self) {

            Self::EqualsOperation(left, right) => {
                let left  = left  .verify();
                let right = right .verify();
                if (left.matches_type(&right)) {
                        left.equals(&right)
                } else {
                    Value::Bool(ValConstr::failed())
                }
            },

            Self::NotEqualsOperation(_, _) => {
                todo!()
            },

            Self::GreaterOperation(_, _) => {
                todo!()
            },

            Self::GreaterEqualsOperation(_, _) => {
                todo!()
            },

            Self::LessOperation(_, _) => {
                todo!()
            },

            Self::LessEqualsOperation(_, _) => {
                todo!()
            },

            Self::AdditionOperation(_, _) => {
                todo!()
            },

            Self::SubtractionOperation(_, _) => {
                todo!()
            },

            Self::MultiplicationOperation(_, _) => {
                todo!()
            },

            Self::DivisionOperation(_, _) => {
                todo!()
            },

            Self::Atom(atom) => atom.verify()

        }
    }
}


impl Atom {
    fn verify(&self) -> Value {
        return match (self) {

            Self::Literal(lit) => lit.verify(),
            
            Self::Expression(expr) => expr.verify(),

            Self::If(ifs, els) => {
                for (condition, block) in ifs {
                    // TODO : Check for different return types.
                    let condition = condition.verify();
                    if let Value::Bool(condition) = condition {
                        match (condition.test(&true)) {
                            TestResponse::Always => {
                                push_warn!(BlockContents_Called, Always, "Condition always succeeds. Consider removing if condition?");
                                return block.verify(None);
                            },
                            TestResponse::Never => {
                                push_warn!(BlockContents_Called, Never, "Condition always fails. Consider removing case?");
                            },
                            TestResponse::Sometimes => {
                                let retn = block.verify(None);
                            },
                            TestResponse::Failed => {}
                        }
                    } else {
                        // TODO : PROPER ERROR
                        panic!("condition is not boolean");
                    }
                }
                if let Some(els) = els {
                    return els.verify(None);
                }
                return Value::Void;
            }
            
        }
    }
}


impl Literal {
    fn verify(&self) -> Value {
        return match (self) {

            Self::Int(val) => Value::Int(ValConstrOrd(vec![ValConstrRange::Exact(
                ValuePossiblyBigInt::from(val)
            )])),

            Self::Float(int, dec) => Value::Float(ValConstrOrd(vec![ValConstrRange::Exact(
                ValuePossiblyBigFloat::from(&format!("{}.{}", int, dec))
            )])),

            Self::Identifier(name) => Scope::get_symbol(name).value.clone()

        }
    }
}


impl Block {
    fn verify(&self, name : Option<&String>) -> Value {
        Scope::enter_subscope(name);
        let mut ret   = Value::Void;
        for stmt in &self.stmts {
            ret = stmt.verify();
        }
        Scope::exit_subscope();
        return if (self.retlast) {ret} else {Value::Void};
    }
}
