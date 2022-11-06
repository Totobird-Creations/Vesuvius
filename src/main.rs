#![allow(unused_parens)]


pub mod parse;
pub mod run;


fn main() {
    let fname   = "examples/basic.vsv";
    println!("\n\x1b[36mCompiling\x1b[0m `\x1b[36m\x1b[1m{}\x1b[0m`.\n", fname);
    let script  = parse::read(fname);
    let program = parse::parse(&script);
    run::reset();
    program.verify();
    run::notes::dump(&script);
}
