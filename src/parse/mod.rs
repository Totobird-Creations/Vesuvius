pub mod node;
    mod grammer;
    mod node_fmt;
pub mod config;

use std::fs::read_to_string;

use relative_path::RelativePathBuf;

use crate::{
    parse::node::{
        Range,
        Program
    },
    notes::push_error,
    scope::ProgramInfo
};


fn read(importer : &Option<Range>, base : &RelativePathBuf, module : &Vec<String>) -> Option<String> {
    let mut path = base.clone();
    for part in module {
        path.push(part);
    }
    return match (read_to_string(&path.with_extension("vsv").as_str())) {
        Ok(script) => Some(script),
        Err(error) => {
            push_error!(ModuleNotFound, Always, {
                importer.clone() => {"{}", error},
                None             => {"Module `{}` failed to load.", module.join("::")}
            });
            None
        }
    };
}

fn parse(text : &str, module : Vec<String>) -> Option<Program> {
    return grammer::parse(text.into(), &module)
        .map_err(|e| {push_error!(UnexpectedToken, Always, {
            Some(Range(module, e.location.offset, e.location.offset)) => {"{}.", {
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


pub(crate) fn get_all_modules(importer : Option<Range>, base : &RelativePathBuf, module : Vec<String>) {
    if let Some(script) = read(&importer, base, &module) {
        ProgramInfo::get().add_module(module.clone(), script.clone());
        if let Some(program) = parse(&script, module.clone()) {
            ProgramInfo::get().load_module(base, module, program);
        }
    }
}
