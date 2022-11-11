use crate::parse::node::*;



impl Program {

    pub fn verify(&self) {
        for decl in &self.decls {
            decl.register();
        }
    }

}



impl Declaration {

    fn register(&self) {
        todo!();
    }

}
