use crate::parser::node::*;
use crate::run::types::*;


// pre_verify : Define that the name of the value is taken.
// mid_verify : Convert all type descriptors to value types.
// verify     : Check contents for errors.


impl Program {
    pub fn verify(&self) {
        for decl in &self.decls {
            decl.verify();
        }
    }
}


impl Declaration {
    pub fn verify(&self) {
        match (&self.decl) {
            
            DeclarationType::Function(_, _, _, block) => {
                block.verify();
            }

        }
    }
}


impl Statement {
    pub fn verify(&self) -> Value {
        match (self) {

            Statement::Expression(expr) => {
                expr.verify()
            },

            Statement::InitVar(_, _) => {
                todo!();
            }

        }
    }
}


impl Expression {
    pub fn verify(&self) -> Value {
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
                atom.verify()
            }

        }
    }
}


impl Atom {
    pub fn verify(&self) -> Value {
        return match (self) {

            Self::Literal(lit) => lit.verify(),
            
            Self::Expression(expr) => expr.verify(),

            Self::If(_, _) => {
                todo!();
            }
            
        }
    }
}


impl Literal {
    pub fn verify(&self) -> Value {
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
    pub fn verify(&self) -> Value {
        for stmt in &self.stmts {
            stmt.verify();
        }
        return Value::Void;
    }
}
