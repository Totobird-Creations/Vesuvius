#![allow(unused_parens)]

pub mod parser;
pub mod run;


fn main() {
    let program = parser::parse("examples/basic.vsv");
    run::verify::reset();
    let scope   = run::verify::Scope::new();
    let scope   = program.verify(scope);
    println!("{:?}", scope);
}
