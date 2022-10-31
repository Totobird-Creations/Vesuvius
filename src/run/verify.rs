use std::collections::HashMap;

use crate::parser::node::*;
use crate::run::types::*;


// pre_verify : Define that the name of the value is taken.
// mid_verify : Convert all type descriptors to value types.
// verify     : Check contents for errors.


pub enum ContextParent<'l> {
    Global(&'l mut GlobalContext),
    Simple(&'l mut Context<'l>)
}
pub struct GlobalContext {
    entry : Option<Vec<String>>
}
pub struct Context<'l> {
    pub module  : Vec<String>,
    pub symbols : HashMap<String, (DeclarationVisibility, Value)>,
    pub parent  : ContextParent<'l>
}
impl<'l> Context<'l> {
    fn subcontext(&'l mut self) -> Self {
        return Self {
            module  : self.module.clone(),
            symbols : HashMap::new(),
            parent  : ContextParent::Simple(self)
        };
    }
}


impl Program {
    pub fn verify<'l>(&self, global : &'l mut GlobalContext) {
        let mut context = Context {
            module  : Vec::new(),
            symbols : HashMap::new(),
            parent  : ContextParent::Global(global)
        };
        for decl in &self.decls {
            decl.pre_verify(&mut context);
        }
        for decl in &self.decls {
            decl.mid_verify(&mut context);
        }
        for decl in &self.decls {
            decl.verify(&mut context);
        }
        println!("{:?}", context.symbols);
    }
}



impl Declaration {

    pub fn pre_verify<'l>(&self, context : &'l mut Context<'l>) {
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

    pub fn mid_verify<'l>(&self, context : &'l mut Context<'l>) {
        self.decl.mid_verify(context);
    }

    pub fn verify<'l>(&self, context : &'l mut Context<'l>) {
        self.decl.verify(context);
    }

}


impl DeclarationType {

    pub fn pre_verify<'l>(&self, vis : DeclarationVisibility, context : &'l mut Context<'l>) {
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

    pub fn mid_verify<'l>(&self, context : &'l mut Context<'l>) {
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

    pub fn verify<'l>(&self, context : &'l mut Context<'l>) {
        
        match (&self) {

            DeclarationType::Function(name, _, _, _) => {
                if let Value::FuncType(args, ret, body) = context.symbols.get(name).unwrap().1.clone() {
                    let mut subcontext = context.subcontext();
                    let mut done_args  = Vec::new();
                    for arg in args.iter() {
                        if (done_args.contains(&&arg.0)) {
                            // TODO : Proper error
                            panic!("Duplicate argument name.");
                        }
                        done_args.push(&arg.0);
                        subcontext.symbols.insert(arg.0.clone(), (DeclarationVisibility::Public, arg.1.clone()));
                        for stmt in &body.stmts {
                            stmt.pre_verify(&mut subcontext);
                        }
                        for stmt in &body.stmts {
                            stmt.mid_verify(&mut subcontext);
                        }
                        for stmt in &body.stmts {
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

    pub fn pre_verify<'l>(&self, context : &'l mut Context<'l>) {
        todo!();
    }

    pub fn mid_verify<'l>(&self, context : &'l mut Context<'l>) {
        todo!();
    }

    pub fn verify<'l>(&self, context : &'l mut Context<'l>) {
        todo!();
    }
    
}