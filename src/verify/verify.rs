use crate::{
    notes::{
        push_error,
        push_warn
    },
    parse::node::*,
    verify::{
        types::{
            Value,
            ValueType,
            constr::{
                TestResponse,
                eq::ValConstr,
                ord::{
                    ValConstrOrd,
                    ValConstrRange
                },
                ValConstrState
            }
        },
        scope::{
            self,
            Scope,
            Symbol
        }
    }
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

            DeclarationType::Function(name, range, _, _, block) => {
                // Add the function to the scope.
                Scope::add_symbol(name, Symbol::from(Value {
                    value : ValueType::Function(
                        Box::new(Vec::new()), Box::new(None), block.clone()
                    ),
                    range : *range
                }));
                // If another entry function is already defined, throw an error, otherwise set it.
                for header in &self.headers {
                    match (&header.header) {
                        DeclarationHeaderType::Entry => {
                            let mut lock = scope::PROGRAM_INFO.write();
                            if let Some((range, _)) = lock.entry {
                                push_error!(DuplicateEntryHeader, Always, {
                                    range        => {"Already used here."},
                                    header.range => {"Used again here."}
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

            DeclarationType::Function(name, _, _, _, block) => {
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

            StatementType::InitVar(name, range, value) => {
                let mut value = value.verify();
                value.range = *range;
                Scope::add_symbol(&name, Symbol::from(value));
                return Value {
                    value : ValueType::Void,
                    range : self.range
                };
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
                    Value::bool_from(left_val.eq(&right_val), Range(left_val.range.0, right_val.range.1))
                } else {
                    push_error!(InvalidTypeReceived, Always, {
                        left.range  => {"Left side is of type `{}`.", left_val.type_def()},
                        right.range => {"Both sides must be of the same type, but the right side is of type `{}`.", right_val.type_def()}
                    });
                    Value {
                        value : ValueType::Bool(ValConstr(ValConstrState::Failed)),
                        range : self.range
                    }
                }
            },

            ExpressionType::NotEqualsOperation(left, right) => {
                let left_val  = left  .verify();
                let right_val = right .verify();
                if (left_val.matches_type(&right_val)) {
                    Value::bool_from(left_val.ne(&right_val), Range(left_val.range.0, right_val.range.1))
                } else {
                    push_error!(InvalidTypeReceived, Always, {
                        left.range  => {"Does not match type of right side."},
                        right.range => {"Does not match type of left side."}
                    });
                    Value {
                        value : ValueType::Bool(ValConstr(ValConstrState::Failed)),
                        range : self.range
                    }
                }
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
                let mut ret : Option<Value> = None;
                for i in 0..ifs.len() {
                    let (condition, block, range) = &ifs[i];
                    let cond_val                  = condition.verify();
                    if let ValueType::Bool(cond_val) = cond_val.value {
                        match (cond_val.eq(&true)) {

                            TestResponse::Always => {
                                push_warn!(BlockContents_Called, Always, {
                                    condition.range => {"Condition always succeeds."},
                                    *range          => {"Consider {}?", if (i == 0) {"removing if statement"} else {"replacing this case with else"}}
                                });
                                let mut value = block.verify(None);
                                if let Some(ret_value) = &mut ret {
                                    if (ret_value.matches_type(&value)) {
                                        let mut ret = ret_value.combine(&value);
                                        ret.range   = Range(ret.range.0, range.1);
                                        return ret;
                                    } else {
                                        push_error!(InvalidTypeReceived, Always, {
                                            ret_value.range => {"First case returns `{}`" , ret_value.type_def() },
                                            value.range     => {"This case returns `{}`"     , value.type_def()     }
                                        })
                                    }
                                } else {
                                    value.range = *range;
                                    return value;
                                }
                            },

                            TestResponse::Never => {
                                push_warn!(BlockContents_Called, Never, {
                                    condition.range => {"Condition always fails."},
                                    *range          => {"Consider removing case?"}
                                });
                            },

                            TestResponse::Sometimes => {
                                let mut value = block.verify(None);
                                if let Some(ret_value) = &mut ret {
                                    if (ret_value.matches_type(&value)) {
                                        *ret_value      = ret_value.combine(&value);
                                        ret_value.range = Range(ret_value.range.0, range.1);
                                    } else {
                                        push_error!(InvalidTypeReceived, Always, {
                                            ret_value.range => {"First case returns `{}`" , ret_value.type_def() },
                                            *range          => {"This case returns `{}`"     , value.type_def()     }
                                        })
                                    }
                                } else {
                                    value.range = *range;
                                    ret         = Some(value);
                                }
                            },

                            TestResponse::Failed => {}

                        }
                    } else {
                        push_error!(InvalidTypeReceived, Always, {
                            condition.range => {"Must be of type `bool`, but got `{}`", cond_val.type_def()}
                        });
                    }
                }
                if let Some((block, range)) = els {
                    let mut value = block.verify(None);
                    if let Some(ret_value) = &mut ret {
                        if (ret_value.matches_type(&value)) {
                            *ret_value         = ret_value.combine(&value);
                            (*ret_value).range = *range;
                            return ret_value.clone();
                        } else {
                            push_error!(InvalidTypeReceived, Always, {
                                ret_value.range => {"Previous case returns `{}`" , ret_value.type_def() },
                                value.range     => {"Else case returns `{}`"     , value.type_def()     }
                            })
                        }
                    } else {
                        value.range = *range;
                        return value;
                    }
                } else if let Some(ret_value) = &ret {
                    if (! matches!(ret_value.value, ValueType::Void)) {
                        push_error!(InvalidTypeReceived, Always, {
                            ret_value.range                   => {"Previous case returns `{}`" , ret_value.type_def() },
                            Range(self.range.1, self.range.1) => {"Lack of else case imlpies `void`"                  }
                        })
                    }
                }
                return if let Some(ret_value) = ret {
                    return ret_value;
                } else {
                    Value {
                        value : ValueType::Void,
                        range : self.range
                    }
                };
            }
            
        }
    }
}


impl Literal {
    fn verify(&self) -> Value {
        return Value {
            value : match (&self.lit) {

                LiteralType::Int(val) => ValueType::Int64(ValConstrOrd(ValConstrState::Some(vec![ValConstrRange::Exact(
                    // TODO : Check parse failed.
                    val.parse().unwrap()
                )]))),

                LiteralType::Float(int, dec) => ValueType::Float64(ValConstrOrd(ValConstrState::Some(vec![ValConstrRange::Exact(
                    // TODO : Check parse failed.
                    format!("{}.{}", int, dec).parse().unwrap()
                )]))),

                LiteralType::Identifier(name) => {
                    if (name == "rand_bool") {
                        ValueType::Bool(ValConstr(ValConstrState::Some(vec![true])))
                    } else if let Some(symbol) = Scope::get_symbol(name) {
                        // TODO : Movable
                        symbol.value.value.clone()
                    } else {
                        push_error!(UnknownSymbol, Always, {
                            self.range => {"Symbol is not found in current scope."}
                        });
                        ValueType::Unknown
                    }
                }

            },
            range : self.range
        }
    }
}


impl Block {
    fn verify(&self, name : Option<&String>) -> Value {
        Scope::enter_subscope(name);
        let mut ret = Value {
            value : ValueType::Void,
            range : self.range
        };
        for stmt in &self.stmts {
            let value = stmt.verify();
            if (self.retlast) {
                ret = value;
            }
        }
        Scope::exit_subscope();
        return ret;
    }
}
