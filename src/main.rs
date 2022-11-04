#![allow(unused_parens)]

pub mod parser;
pub mod run;


fn main() {
    let     program = parser::parse("examples/basic.vsv");
    let mut scope   = run::verify::Scope::new();
    program.verify(&mut scope);
    println!("{:?}", scope);
}
