use std::fs;

pub mod node;
    mod grammer;
    mod node_fmt;


pub fn parse<S : Into<String>>(file : S) {
    let file = file.into();
    let text = fs::read_to_string(file).unwrap();
    println!("{}", match (grammer::parse(text)) {
        Ok(v)  => format!("{}", v),
        Err(e) => format!("{}", e)
    });
}
