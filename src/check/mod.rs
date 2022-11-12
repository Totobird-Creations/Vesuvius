use crate::parse::node::*;



impl Program {

    pub(crate) fn _check(&self) {
        for decl in &self.decls {
            decl._register();
        }
    }

}



impl Declaration {

    fn _register(&self) {
        todo!();
    }

}
