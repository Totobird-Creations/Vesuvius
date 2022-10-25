use std::fs;

use peg;


pub struct Declaration {
    headers : Vec<HeaderType>,
}
impl Declaration {
    pub fn from(headers : Vec<HeaderType>) -> Declaration {
        return Declaration {
            headers : headers
        };
    }
    pub fn to_header(headers : Vec<&str>) -> Vec<HeaderType> {
        return headers.iter().map(|h| match *h {
            "entry" => HeaderType::Entry,
            "pub"   => HeaderType::Public,
            _       => panic!("INTERNAL ERROR")
        }).collect::<Vec<HeaderType>>();
    }
}
pub enum HeaderType {
    Entry,
    Public
}


pub fn read(path : &str) -> Vec<Declaration> {
    return parser::program(&fs::read_to_string(path).unwrap()).unwrap();
}


peg::parser! {
    grammar parser() for str {

        pub rule program() -> Vec<Declaration>
            = e:(declaration() ** ";")
            {e}

        rule declaration() -> Declaration
            = h:declaration_header()
            {Declaration::from(h)}

        rule declaration_header() -> Vec<HeaderType>
            = h:$("#[" ("entry" / "pub") "]")*
            {Declaration::to_header(h)}
    }
}
