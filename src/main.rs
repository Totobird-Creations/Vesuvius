#![allow(unused_parens)]
#![feature(decl_macro)]


pub mod parser;
pub mod run;


fn main() {
    parser::parse("examples/basic.vsv");
}
