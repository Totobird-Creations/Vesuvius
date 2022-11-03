#![allow(unused_parens)]


pub mod parser;
pub mod run;


fn main() {
    let program = parser::parse("examples/basic.vsv");
    program.verify();
}
