#![allow(unused_parens)]


pub mod parser;
pub mod run;


fn from_file<S : Into<String>>(name : S) -> parser::node::Program {
    return parser::parse(parser::read(name.into()));
}


fn main() {
    let program = parser::parse(parser::read("examples/basic.vsv"));
    run::reset();
    program.verify();
    run::notes::dump();
    println!("{:?}", unsafe{&run::scope::SCOPE}[0]);
}
