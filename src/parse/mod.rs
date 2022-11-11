pub mod node;
    mod grammer;
    mod node_fmt;

use std::{
    fs::read_to_string,
    path::PathBuf,
};

use crate::{
    parse::node::{
        Range,
        Program
    },
    notes::push_error,
    scope::ProgramInfo
};


fn read(importer : &Option<Range>, path : &PathBuf) -> Option<String> {
    return match (read_to_string(path.with_extension("vsv"))) {
        Ok(script) => Some(script),
        Err(error) => {
            push_error!(ModuleNotFound, Always, {
                importer.clone() => {"{}", error},
                None             => {"Module `{}` failed to load.", path.to_str().unwrap()}
            });
            None
        }
    };
}

fn parse(text : &str, path : PathBuf) -> Option<Program> {
    return grammer::parse(text.into(), &path)
        .map_err(|e| {push_error!(UnexpectedToken, Always, {
            Some(Range(path, e.location.offset, e.location.offset)) => {"{}.", {
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


pub(crate) fn get_all_modules(importer : Option<Range>, path : PathBuf) {
    if let Some(script) = read(&importer, &path) {
        ProgramInfo::get().add_module(path.clone(), script.clone());
        if let Some(program) = parse(&script, path.clone()) {
            ProgramInfo::get().load_module(path, program);
        }
    }
}
