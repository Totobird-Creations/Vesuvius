#![allow(unused_parens, non_snake_case)]

pub mod notes;
pub mod parse;
pub mod verify;

use std::{
    io::{
        stdout,
        Write
    },
    process::exit
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
        &String::new() => reset()
    };

    let fname = "examples/basic.vsv";
    let script = parse::read(fname);
    let program = attempt!{
        "Parsing";
        &script => parse::parse(&script)
    }.unwrap();

    attempt!{
        "Verifying";
        &script => program.verify("program")
    };

    /*attempt!{
        "Compiling";
        fin &script => notes::push_error!(InternalError, Always, {
            parse::node::Range(0, 0) => {"Todo : Compile"}
        })
    };*/

}


macro_rules! attempt {
    {$title:expr; fin $script:expr => $expr:expr} => {
        $crate::attempt!{$title; true $script => $expr}
    };
    {$title:expr; $script:expr => $expr:expr} => {
        $crate::attempt!{$title; false $script => $expr}
    };
    {$title:expr; $fin:ident $script:expr => $expr:expr} => {{
        printw!("\n \x1b[37m\x1b[2m=>\x1b[0m \x1b[96m{}\x1b[0m\x1b[36m\x1b[2m...\x1b[0m", $title);
        let v = $expr;
        match ($crate::notes::dump(4 + $title.len() + 13, $fin, $script)) {
            Ok(text) => {
                printw!(" [\x1b[32m\x1b[1mSUCCESS\x1b[0m]\n");
                printw!("{}", text);
            },
            Err(text) => {
                printw!(" [\x1b[31m\x1b[1mFAILURE\x1b[0m]\n");
                printw!("{}", text);
                $crate::exit(1);
            }
        };
        v
    }};
}
use attempt;


macro_rules! printw {
    ($($tt:tt)*) => {{
        let mut stdout = stdout();
        write!(stdout, $($tt)*).unwrap();
        stdout.flush().unwrap();
    }}
}
use printw;
