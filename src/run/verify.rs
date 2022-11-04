use std::collections::HashMap;

use crate::parser::node::*;
use crate::run::types::*;


#[derive(Debug)]
pub struct ProgramInfo {
    entry : Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ScopeParent<'l> {
    Global(ProgramInfo),
    Scope(&'l mut Scope<'l>)
}

#[derive(Debug)]
pub struct Scope<'l> {
    name    : String,
    parent  : ScopeParent<'l>,
    symbols : HashMap<String, Symbol>
}
impl<'l> Scope<'l> {
    pub fn new() -> Scope<'l> {
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
    pub fn subscope(&mut self, name : Option<&String>) -> Scope {
        return Scope {
            name    : name.unwrap_or_else(||&self.name).clone(),
            parent  : ScopeParent::Scope(self),
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


impl Program {
    pub fn verify(&self, scope : &mut Scope) {
        for decl in &self.decls {
            decl.verify_register(scope);
        }
        for decl in &self.decls {
            decl.verify_contents(scope);
        }
    }
}


impl Declaration {

    // Register that the function exists, so that other objects can reference it.
    pub fn verify_register(&self, scope : &mut Scope) {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                scope.symbols.insert(name.clone(), Symbol::from(Value::Function(
                    Box::new(Vec::new()), Box::new(None), block.clone()
                )));
            }

        }
    }

    pub fn verify_contents(&self, scope : &mut Scope) {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, block) => {
                block.verify(scope, Some(name));
            }

        }
    }

}


impl Statement {
    pub fn verify(&self, scope : &mut Scope) -> Value {
        return match (self) {

            Statement::Expression(expr) => {
                expr.verify(scope)
            },

            Statement::InitVar(name, value) => {
                let symbol = Symbol::from(value.verify(scope));
                scope.symbols.insert(name.clone(), symbol);
                Value::Void
            }

        }
    }
}


impl Expression {
    pub fn verify(&self, scope : &mut Scope) -> Value {
        return match (self) {

            Self::EqualsOperation(_, _) => {
                todo!()
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
    pub fn verify(&self, scope : &mut Scope) -> Value {
        return match (self) {

            Self::Literal(lit) => lit.verify(scope),
            
            Self::Expression(expr) => expr.verify(scope),

            Self::If(_, _) => {
                todo!();
            }
            
        }
    }
}


impl Literal {
    pub fn verify(&self, _scope : &mut Scope) -> Value {
        return match (self) {

            Self::Int(val) => Value::Int(ValConstrOrd(vec![ValConstrRange::Exact(
                ValuePossiblyBigInt::from(val)
            )])),

            Self::Float(int, dec) => Value::Float(ValConstrOrd(vec![ValConstrRange::Exact(
                ValuePossiblyBigFloat::from(&format!("{}.{}", int, dec))
            )])),

            Self::Identifier(_) => {
                todo!();
            }

        }
    }
}


impl Block {
    pub fn verify(&self, scope : &mut Scope, name : Option<&String>) -> Value {
        let mut scope = scope.subscope(name);
        for stmt in &self.stmts {
            stmt.verify(&mut scope);
        }
        return Value::Void;
    }
}
