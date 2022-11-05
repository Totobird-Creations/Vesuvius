use std::{
    fs,
    process
};

pub mod node;
    mod grammer;
    mod node_fmt;


pub fn read<S : Into<String>>(file : S) -> String {
    return fs::read_to_string(file.into()).unwrap();
}

pub fn parse<S : Into<String>>(text : S) -> node::Program {
    match (grammer::parse(text.into())) {
        Ok(v) => {
            return v;
        },
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
}
