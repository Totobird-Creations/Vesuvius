//! ---
//! 
//! ### Parser and compiler for the Vesuvius Programming Language.
//! by [Totobird Creations](https://github.com/Totobird-Creations/)
//! 
//! ---
//! 
//! Github : [Totobird-Creations/Vesuvius](https://github.com/Totobird-Creations/Vesuvius/)
//! 
//! ---
//! 
//! > This is the documentation for the internals of vesuvius.
//! > If you are a user of the language and not a developer, you are probably in the wrong place. See the docs (COMING SOON).

#![feature(absolute_path, decl_macro)]
#![allow(unused_parens)]

            mod cli;
pub (crate) mod notes;
pub (crate) mod scope;
pub (crate) mod parse;
pub (crate) mod verify;

use std::{
    process::exit,
    path::PathBuf
};

use clap::Parser;

use {
    cli::Cli,
    scope::Scope
};


/// Reset all systems, ready for parsing and compilation.
fn reset() {
    // Reset scope system.
    Scope::reset();
    // If debug, add unstable version warning.
    #[cfg(debug_assertions)]
    notes::push_warn!(UnstableVersion, Always);
}


/// Entry point of the program.
fn main() {

    let cli = Cli::parse();
    cli.run();

    /*attempt!{
        "Preparing";
        reset()
    };

    let fname = PathBuf::from("./examples/basic/main");
    attempt!{
        "Parsing";
        parse::get_all_modules(None, fname)
    };

    attempt!{
        "Compiling";
        fin &script => notes::push_error!(InternalError, Always, {
            parse::node::Range(0, 0) => {"Todo : Compile"}
        })
    };*/

}


/// Print a title, run a function, and report any warnings and/or errors.
/// If any errors were emitted, exit the program.
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


/// Print and flush some text to the console without a newline.
/// Almost identical to `println!`.
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
