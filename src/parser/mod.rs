use std::{
    fs,
    process
};

pub mod node;
    mod grammer;
    mod node_fmt;


pub fn parse<S : Into<String>>(file : S) -> node::Program {
    let file = file.into();
    let text = fs::read_to_string(file).unwrap();
    match (grammer::parse(text)) {
        Ok(v)  => {
            println!("{}", v);
            return v;
        },
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
}
