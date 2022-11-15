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

#![feature(absolute_path, decl_macro, let_chains)]
#![allow(unused_parens)]

            mod cli;
pub (crate) mod notes;
pub (crate) mod scope;
pub (crate) mod parse;
pub (crate) mod check;
pub (crate) mod helper;

use clap::Parser;

use cli::Cli;


/// Reset all systems, ready for parsing and compilation.
fn reset() {
    // Reset scope system.
    scope::reset();
    // If debug, add unstable version warning.
    #[cfg(debug_assertions)]
    notes::push_warn!(UnstableVersion, Always);
}


/// Entry point of the program.
fn main() {

    let cli = Cli::parse();
    cli.handle();

}
