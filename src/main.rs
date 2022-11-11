#![feature(absolute_path, decl_macro)]
#![allow(unused_parens, non_snake_case)]

pub mod notes;
pub mod parse;
pub mod verify;

use std::{
    process::exit,
    path::PathBuf
};

use verify::scope::Scope;


fn reset() {
    // Reset notes.
    {
        let mut lock = notes::COMPILATION_NOTES.write();
        lock.clear();
    }
    // Reset scope system.
    Scope::reset();
    // If debug, add unstable version warning.
    #[cfg(debug_assertions)]
    notes::push_warn!(UnstableVersion, Always);
}


fn main() {

    attempt!{
        "Preparing";
        reset()
    };

    let fname = PathBuf::from("./examples/basic/main");
    attempt!{
        "Parsing";
        parse::get_all_modules(None, fname)
    };

    println!("\n");
    for (file, program) in verify::scope::ProgramInfo::get().modules() {
        println!("\n\n{:?}\n", file);
        println!("{}", program);
    }

    /*attempt!{
        "Compiling";
        fin &script => notes::push_error!(InternalError, Always, {
            parse::node::Range(0, 0) => {"Todo : Compile"}
        })
    };*/

}


macro attempt {
    {$title:expr; fin; $expr:expr} => {
        $crate::attempt!{$title; true; $expr}
    },
    {$title:expr; $expr:expr} => {
        $crate::attempt!{$title; false; $expr}
    },
    {$title:expr; $fin:ident; $expr:expr} => {{
        $crate::printw!("\n \x1b[37m\x1b[2m=>\x1b[0m \x1b[96m{}\x1b[0m\x1b[36m\x1b[2m...\x1b[0m", $title);
        let v = $expr;
        match ($crate::notes::dump(4 + $title.len() + 13, $fin)) {
            Ok(text) => {
                $crate::printw!(" [\x1b[32m\x1b[1mSUCCESS\x1b[0m]\n");
                $crate::printw!("{}", text);
            },
            Err(text) => {
                $crate::printw!(" [\x1b[31m\x1b[1mFAILURE\x1b[0m]\n");
                $crate::printw!("{}", text);
                $crate::exit(1);
            }
        };
        v
    }}
}


macro printw {
    ($($tt:tt)*) => {{
        use std::io::{
            stdout,
            Write
        };
        let mut stdout = stdout();
        write!(stdout, $($tt)*).unwrap();
        stdout.flush().unwrap();
    }}
}
