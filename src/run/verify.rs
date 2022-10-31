use std::collections::HashMap;

use crate::parser::node::*;
use crate::run::types::*;



impl Program {
    pub fn verify(&self) {
        let mut context = Context {
            entry   : None,
            module  : Vec::new(),
            symbols : HashMap::new()
        };
        for decl in &self.decls {
            decl.pre_verify(&mut context);
        }
        println!("{:?}", context.symbols);
    }
}
