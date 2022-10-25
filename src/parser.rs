use std::fs;

use peg;


pub struct Declaration {
    headers : Vec<HeaderType>,

}
pub enum HeaderType {
    Entry
}


pub fn read(path : &str) -> Vec<Declaration> {
    return parser::program(fs::read_to_string(path)).unwrap();
}


peg::parser! {
    grammar parser() for String {
        use super::Nodes;

        pub program() -> Vec<Nodes>
            = e:(declaration ";")* {e}
    }
}
