pub mod node;
pub mod grammer;
pub mod node_fmt;

use std::fs;

use crate::notes::push_error;


pub fn read<S : Into<String>>(file : S) -> String {
    return fs::read_to_string(file.into()).unwrap();
}

pub fn parse<S : Into<String>>(text : S) -> Option<node::Program> {
    return grammer::parse(text.into())
        .map_err(|e| {push_error!(UnexpectedToken, Always, {
            node::Range(e.location.offset, e.location.offset) => {"{}.", {
                let mut tokens = e.expected.tokens().collect::<Vec<_>>();
                if (tokens.len() == 1) {
                    format!("Expected {}", tokens[0])
                } else if (tokens.len() == 2) {
                    format!("Expected \x1b[91m{}\x1b[31m or \x1b[91m{}\x1b[31m", tokens[0], tokens[1])
                } else {
                    let last = tokens.remove(tokens.len() - 1);
                    format!("Expected one of {}, or \x1b[91m{}\x1b[31m",
                        tokens.iter()
                            .map(|token| format!("\x1b[91m{}\x1b[31m", token))
                            .collect::<Vec<_>>().join(", "),
                        last
                    )
                }
            }}
        }); e})
        .ok();
}
