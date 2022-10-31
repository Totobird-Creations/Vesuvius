#![allow(unused_parens)]
#![feature(decl_macro)]


pub mod parser;
pub mod run;


fn main() {
    let     program = parser::parse("examples/basic.vsv");
    let mut context = run::verify::GlobalContext {
        entry : None
    };
    program.verify(&mut context);
}
