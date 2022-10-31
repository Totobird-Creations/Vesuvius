use std::collections::HashMap;

use crate::parser::node::*;
use crate::run::types::*;


// pre_verify : Define that the name of the value is taken.
// mid_verify : Convert all type descriptors to value types.
// verify     : Check contents for errors.


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
        for decl in &self.decls {
            decl.mid_verify(&mut context);
        }
        for decl in &self.decls {
            decl.verify(&context);
        }
        println!("{:?}", context.symbols);
    }
}



impl Declaration {

    pub fn pre_verify(&self, context : &mut Context) {
        match (&self.decl) {

            DeclarationType::Function(name, _, _, _) => {
                for header in &self.headers {
                    if let DeclarationHeader::Entry = header {
                        todo!();
                    } else {
                        // TODO : ADD PROPER ERROR
                        panic!("Header `{}` can not be used on functions.", header.format(0));
                    }
                }
            }

        }

        self.decl.pre_verify(self.vis, context)
    }

    pub fn mid_verify(&self, context : &mut Context) {
        self.decl.mid_verify(context);
    }

    pub fn verify(&self, context : &Context) {
        self.decl.verify(context);
    }

}


impl DeclarationType {

    pub fn pre_verify(&self, vis : DeclarationVisibility, context : &mut Context) {
        match (self) {

            Self::Function(name, _, _, block) => {
                context.symbols.insert(name.clone(), (vis,
                    Value::FuncType(
                        Box::new(Vec::new()),
                        Box::new(None),
                        block.clone()
                    )
                ));
            }

        }
    }

    pub fn mid_verify(&self, context : &mut Context) {
        match (self) {

            Self::Function(name, args, ret, _) => {
                if let Value::FuncType(ref mut vargs, ref mut vret, _) = &mut context.symbols.get_mut(name).unwrap().1 {
                    *vargs = Box::new(
                        args.clone().iter()
                            .map(|(argname, arg)| (argname.clone(), into_value_type(arg)))
                            .collect()
                    );
                    *vret  = Box::new(ret.clone().map_or(None, |ret|Some(into_value_type(&ret))));
                } else {
                    panic!("INTERNAL ERROR");
                }
            }

        }
    }

    pub fn verify(&self, context : &Context) {
        
        match (&self) {

            DeclarationType::Function(name, _, _, _) => {
                if let Value::FuncType(args, ret, body) = &context.symbols.get(name).unwrap().1 {
                    let mut subcontext = context.clone();
                    let mut done_args  = Vec::new();
                    for arg in args.iter() {
                        if (done_args.contains(&&arg.0)) {
                            // TODO : Proper error
                            panic!("Duplicate argument name.");
                        }
                        done_args.push(&arg.0);
                        subcontext.symbols.insert(arg.0.clone(), (DeclarationVisibility::Public, arg.1.clone()));
                        for stmt in body.stmts {
                            stmt.pre_verify(&mut subcontext);
                        }
                        for stmt in body.stmts {
                            stmt.mid_verify(&mut subcontext);
                        }
                        for stmt in body.stmts {
                            stmt.verify(&mut subcontext);
                        }
                    }
                } else {
                    panic!("INTENRAL ERROR");
                }
            }

        }
    }

}



impl Statement {

    pub fn pre_verify(&self, context : &mut Context) {
        todo!();
    }

    pub fn mid_verify(&self, context : &mut Context) {
        todo!();
    }

    pub fn verify(&self, context : &mut Context) {
        todo!();
    }
    
}