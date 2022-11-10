use crate::{
    parse::node::*,
    verify::scope::{
        Scope,
        ScopeGuard
    }
};


impl Program {
    pub fn verify(&self, name : &str) {
        let scope = Scope::new(Some(name));
        for decl in &self.decls {
            decl.register(&scope);
        }
    }
}

impl Declaration {
    fn register(&self, scope : &ScopeGuard) {
        println!("{}", scope.path())
    }
}
