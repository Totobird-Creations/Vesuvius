use crate::{
    notes::{
        push_error,
        push_warn
    },
    parse::node::*,
    verify::{
        scope::{
            self,
            Scope
        }
    }
};


impl Program {
    pub fn verify(&self) {
        push_error!(InternalError, Always, {
            Range(0, 0) => {"Todo."}
        });
    }
}


impl Block {
    fn verify(&self, name : Option<&String>) {
        println!("a");
        let scope = Scope::new(name);
        println!("b");
    }
}
