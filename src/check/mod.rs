pub(crate) mod types;

use crate::{
    parse::node::*,
    scope::LinkedScopes
};



impl Program {

    pub(crate) fn register_declarations(&self, _ : &mut LinkedScopes) {
        self.decls.iter().for_each(|decl| decl.register());
    }

}



impl Declaration {

    fn register(&self) {
        use DeclarationType::*;
        match (&self.decl) {
            Module(_, _) => {},
            Function(_, _, _, _, _) => {}
        }
    }

}
