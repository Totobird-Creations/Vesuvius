use crate::parse::node::*;
use crate::run::types::*;
use crate::run::notes::{
    push_error,
    push_warn
};
use crate::run::scope::{
    self,
    *
};


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
                for header in &self.headers {
                    match (&header.header) {
                        DeclarationHeaderType::Entry => {
                            let mut lock = scope::PROGRAM_INFO.write();
                            if let Some((range, _)) = lock.entry {
                                push_error!(DuplicateEntry, Always, {
                                    range        => {"#[entry] already defined here."},
                                    header.range => {"#[entry] used again here."}
                                });
                            } else {
                                lock.entry = Some((header.range, Scope::module_with_sub(name)));
                            }
                        }
                    }
                }
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
        return match (&self.stmt) {

            StatementType::Expression(expr) => {
                expr.verify()
            },

            StatementType::InitVar(name, value) => {
                Scope::add_symbol(&name, Symbol::from(value.verify()));
                
                return Value::Void;
            }

        }
    }
}


impl Expression {
    fn verify(&self) -> Value {
        return match (&self.expr) {

            ExpressionType::EqualsOperation(left, right) => {
                let left_val  = left  .verify();
                let right_val = right .verify();
                if (left_val.matches_type(&right_val)) {
                        left_val.equals(&right_val)
                } else {
                    push_error!(InvalidTypeReceived, Always, {
                        left.range  => {"Does not match type of right side."},
                        right.range => {"Does not match type of left side."}
                    });
                    Value::Bool(ValConstr::failed())
                }
            },

            ExpressionType::NotEqualsOperation(_, _) => {
                todo!()
            },

            ExpressionType::GreaterOperation(_, _) => {
                todo!()
            },

            ExpressionType::GreaterEqualsOperation(_, _) => {
                todo!()
            },

            ExpressionType::LessOperation(_, _) => {
                todo!()
            },

            ExpressionType::LessEqualsOperation(_, _) => {
                todo!()
            },

            ExpressionType::AdditionOperation(_, _) => {
                todo!()
            },

            ExpressionType::SubtractionOperation(_, _) => {
                todo!()
            },

            ExpressionType::MultiplicationOperation(_, _) => {
                todo!()
            },

            ExpressionType::DivisionOperation(_, _) => {
                todo!()
            },

            ExpressionType::Atom(atom) => atom.verify()

        }
    }
}


impl Atom {
    fn verify(&self) -> Value {
        return match (&self.atom) {

            AtomType::Literal(lit) => lit.verify(),
            
            AtomType::Expression(expr) => expr.verify(),

            AtomType::If(ifs, els) => {
                for i in 0..ifs.len() {
                    let (condition, block) = &ifs[i];
                    // TODO : Check for different return types.
                    let cond_val = condition.verify();
                    if let Value::Bool(cond_val) = cond_val {
                        match (cond_val.test(&true)) {
                            TestResponse::Always => {
                                push_warn!(BlockContents_Called, Always, {
                                    condition.range => {"Condition always succeeds."},
                                    block.range     => {"Consider {}?", if (i == 0) {"removing if statement"} else {"replacing this case with else"}}
                                });
                                /*push_warn!(BlockContents_Called, Always, "{}", if (i == 0) {
                                    "Condition always succeeds. Consider removing if statement?"
                                } else {
                                    "Condition always succeeds. Consider replacing this case with else?"
                                });*/
                                return block.verify(None);
                            },
                            TestResponse::Never => {
                                push_warn!(BlockContents_Called, Never, {
                                    condition.range => {"Condition always fails."},
                                    block.range     => {"Consider removing case?"}
                                });
                            },
                            TestResponse::Sometimes => {
                                block.verify(None);
                            },
                            TestResponse::Failed => {}
                        }
                    } else {
                        push_error!(InvalidTypeReceived, Always, {
                            condition.range => {"Must be of type `bool`"}
                        });
                        return Value::Void;
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
