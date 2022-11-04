use std::collections::HashMap;

use static_init::dynamic;

use crate::parser::node::*;
use crate::run::types::*;
use crate::run::notes::{
    self,
    CompilationNote
};


#[dynamic]
pub static mut compilation_notes : Vec<CompilationNote> = Vec::new();


#[derive(Debug)]
pub struct ProgramInfo {
    entry : Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ScopeParent {
    Global(ProgramInfo),
    Scope(Box<Scope>)
}

#[derive(Debug)]
pub struct Scope {
    name    : String,
    parent  : ScopeParent,
    symbols : HashMap<String, Symbol>
}
impl Scope {
    pub fn new() -> Scope {
        return Scope {
            name    : String::new(),
            parent  : ScopeParent::Global(ProgramInfo {
                entry : None
            }),
            symbols : HashMap::new()
        }
    }
    pub fn get_program_info(&mut self) -> &mut ProgramInfo {
        return match (&mut self.parent) {
            ScopeParent::Global (global) => global,
            ScopeParent::Scope  (scope)  => scope.get_program_info()
        };
    }
    pub fn subscope(self, name : Option<&String>) -> Scope {
        return Scope {
            name    : name.unwrap_or_else(||&self.name).clone(),
            parent  : ScopeParent::Scope(Box::new(self)),
            symbols : HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    value : Value
}
impl Symbol {
    pub fn from(value : Value) -> Symbol {
        return Symbol {
            value
        };
    }
}

struct VerifyResponse {
    scope : Scope,
    value : Value
}


pub fn reset() {
    let mut lock = compilation_notes.write();
    lock.clear();
}
macro_rules! push_error {
    ($typ:ident, $occur:ident, $($text:tt)*) => {{
        let mut lock = compilation_notes.write();
        let     note = CompilationNote {
            occurance : notes::NoteOccurance::$occur,
            level     : notes::NoteType::Error(notes::ErrorType::$typ),
            text      : format!($($text)*)
        };
        lock.push(note);
    }}
}
use push_error;
macro_rules! push_warn {
    ($typ:ident, $occur:ident, $($text:tt)*) => {{
        let mut lock = compilation_notes.write();
        let     note = CompilationNote {
            occurance : notes::NoteOccurance::$occur,
            level     : notes::NoteType::Warn(notes::WarnType::$typ),
            text      : format!($($text)*)
        };
        println!("{}", note);
        lock.push(note);
    }}
}
use push_warn;


impl Program {
    pub fn verify(&self, mut scope : Scope) -> Scope {
        for decl in &self.decls {
            v!{decl.verify_register(scope)};
        }
        for decl in &self.decls {
            v!{decl.verify_contents(scope)};
        }
        return scope;
    }
}


impl Declaration {

    // Register that the function exists, so that other objects can reference it.
    fn verify_register(&self, mut scope : Scope) -> VerifyResponse {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                scope.symbols.insert(name.clone(), Symbol::from(Value::Function(
                    Box::new(Vec::new()), Box::new(None), block.clone()
                )));
            }

        }

        return VerifyResponse {
            scope,
            value : Value::Void
        }
    }

    // Verify the contents of the declaration.
    fn verify_contents(&self, mut scope : Scope) -> VerifyResponse {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                v!{block.verify(scope, Some(name))};
            }

        }

        return VerifyResponse {
            scope,
            value : Value::Void
        }
    }

}


impl Statement {
    fn verify(&self, mut scope : Scope) -> VerifyResponse {
        return match (self) {

            Statement::Expression(expr) => {
                expr.verify(scope)
            },

            Statement::InitVar(name, value) => {
                v!{let value = value.verify(scope)};
                let symbol = Symbol::from(value);
                scope.symbols.insert(name.clone(), symbol);
                
                return VerifyResponse {
                    scope,
                    value : Value::Void
                }
            }

        }
    }
}


impl Expression {
    fn verify(&self, scope : Scope) -> VerifyResponse {
        return match (self) {

            Self::EqualsOperation(_, _) => {
                // TODO
                VerifyResponse {
                    scope,
                    value : Value::Bool(ValConstr(vec![true]))
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

            Self::Atom(atom) => {
                atom.verify(scope)
            }

        }
    }
}


impl Atom {
    fn verify(&self, mut scope : Scope) -> VerifyResponse {
        return match (self) {

            Self::Literal(lit) => lit.verify(scope),
            
            Self::Expression(expr) => expr.verify(scope),

            Self::If(ifs, els) => {
                for (condition, block) in ifs {
                    v!{let condition = condition.verify(scope)};
                    if let Value::Bool(condition) = condition {
                        match (condition.test(&true)) {
                            TestResponse::Always => {
                                push_warn!(BlockContents_Called, Always, "Condition always succeeds. Consider removing if condition?");
                                v!{let retn = block.verify(scope, None)};
                                return VerifyResponse {
                                    scope : scope,
                                    value : retn
                                };
                            },
                            TestResponse::Never => {
                                push_warn!(BlockContents_Called, Never, "Condition always fails. Consider removing case?");
                            },
                            TestResponse::Sometimes => {
                                
                            }
                        }
                    } else {
                        // TODO : PROPER ERROR
                        panic!("condition is not boolean");
                    }
                }
                todo!();
            }
            
        }
    }
}


impl Literal {
    fn verify(&self, scope : Scope) -> VerifyResponse {
        return match (self) {

            Self::Int(val) => VerifyResponse {
                scope,
                value : Value::Int(ValConstrOrd(vec![ValConstrRange::Exact(
                    ValuePossiblyBigInt::from(val)
                )]))
            },

            Self::Float(int, dec) => VerifyResponse {
                scope,
                value : Value::Float(ValConstrOrd(vec![ValConstrRange::Exact(
                    ValuePossiblyBigFloat::from(&format!("{}.{}", int, dec))
                )]))
            },

            Self::Identifier(name) => {
                todo!()
            }

        }
    }
}


impl Block {
    fn verify(&self, scope : Scope, name : Option<&String>) -> VerifyResponse {
        let mut scope = scope.subscope(name);
        let mut ret   = Value::Void;
        for stmt in &self.stmts {
            v!{ret = stmt.verify(scope)};
        }
        return VerifyResponse {
            scope,
            value : if (self.retlast) {ret} else {Value::Void}
        };
    }
}



macro_rules! v {
    {let $value:ident = $obj:ident . $func:ident ($scope:ident $(, $arg:expr)*)} => {
        let VerifyResponse {
            scope : a,
            value : $value
        } = $obj.$func($scope $(, $arg)*);
        $scope = a;
    };
    {$value:ident = $obj:ident . $func:ident ($scope:ident $(, $arg:expr)*)} => {
        let VerifyResponse {
            scope : a,
            value : b
        } = $obj.$func($scope $(, $arg)*);
        $scope = a;
        $value = b;
    };
    {$obj:ident . $func:ident ($scope:ident $(, $arg:expr)*)} => {
        let VerifyResponse {
            scope : a,
            value : _
        } = $obj.$func($scope $(, $arg)*);
        $scope = a;
    };
}
use v;
