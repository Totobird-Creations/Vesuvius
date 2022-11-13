use std::{
    process::exit,
    env::current_dir
};

use clap::{
    Parser,
    Subcommand
};
use relative_path::RelativePathBuf;

use crate::{
    notes::{
        explain,
        push_error
    },
    parse::get_all_modules,
    reset,
    scope::ProgramInfo,
    helper::AbsolutePathBuf
};


pub mod colours {
    pub const PRIMARY   : u8 = 1;
    pub const SECONDARY : u8 = 5;
    pub const TERTIARY  : u8 = 3;
}
pub mod info {
    pub const HOME    : &'static str = "";
    pub const REPO    : &'static str = "https://github.com/Totobird-Creations/Vesuvius/";
    pub const DOCS    : &'static str = "";
    pub const LICENSE : &'static str = "GNU Lesser General Public License v2.1";
}


#[derive(Parser)]
pub(crate) struct Cli {

    #[command(subcommand)]
    command : Option<Command>

}


#[derive(Subcommand)]
enum Command {

    /// Get version info about Vesuvius.
    Version,

    /// Get links to Vesuvius related sites.
    Info,

    /// Check if the program can be compiled.
    Check {
        /// The path containing the entry script (main.vsv).
        /// If none is given, it will use the current working directory.
        path : Option<RelativePathBuf>
    },

    /// Check if the program can be compiled,
    /// then build it.
    Build {
        /// The path containing the entry script (main.vsv).
        /// If none is given, it will use the current working directory.
        path : Option<RelativePathBuf>
    },

    /// Check if the program can be compiled,
    /// build it, then run it.
    Run {
        /// The path containing the entry script (main.vsv).
        /// If none is given, it will use the current working directory.
        path : Option<RelativePathBuf>
    },

    /// Show the documentation for a certain error.
    Explain {
        /// The error code.
        code : String
    }

}



impl Cli {

    pub(crate) fn handle(self) {
        use Command::*;
        match (self.command) {
            None                 => {Cli::version()},
            Some(Version)        => {Cli::version()},
            Some(Info)           => {Cli::info()}
            Some(Explain {code}) => {Cli::explain(code)},
            Some(Check   {path}) => {Cli::check(path)},
            Some(Build   {path}) => {Cli::build(path)},
            Some(Run     {path}) => {Cli::run(path)}
        }
    }


    fn version() {
        use colours::*;
        use info::*;
        {
            let underline = "_".repeat(LICENSE.len());
            println!("   \x1b[3{SECONDARY}m/^\\\x1b[0m  \x1b[9{TERTIARY}m{} \x1b[2mv\x1b[0m\x1b[9{TERTIARY}m\x1b[1m{}\x1b[0m",
                env!("CARGO_PKG_NAME")[0..1].to_uppercase() + &env!("CARGO_PKG_NAME")[1..],
                env!("CARGO_PKG_VERSION")
            );
            println!("  \x1b[3{SECONDARY}m/‾‾‾\\_\x1b[0m  \x1b[3{TERTIARY}m{HOME}\x1b[0m");
            println!(" \x1b[3{SECONDARY}m/\x1b[9{PRIMARY}m‾\\ /‾ \x1b[3{SECONDARY}m\\\x1b[0m   \x1b[3{TERTIARY}m\x1b[2m{LICENSE}\x1b[0m");
            println!("\x1b[3{SECONDARY}m/   \x1b[9{PRIMARY}mV\x1b[3{PRIMARY}mesuvius\x1b[3{SECONDARY}m{underline}\x1b[0m");
        }
    }


    fn info() {
        use colours::*;
        use info::*;
        println!("\x1b[9{SECONDARY}mHomepage\x1b[0m      : \x1b[3{TERTIARY}m{HOME}");
        println!("\x1b[9{SECONDARY}mRepository\x1b[0m    : \x1b[3{TERTIARY}m{REPO}");
        println!("\x1b[9{SECONDARY}mDocumentation\x1b[0m : \x1b[3{TERTIARY}m{DOCS}");
    }


    fn explain(code : String) {
        if let Ok(code) = usize::from_str_radix(&code, 16) {
            if let Some(doc) = explain(code) {
                println!("{}", doc);
                return;
            }
        }
        println!("\x1b[31m\x1b[1merror:\x1b[0m Invalid code `\x1b[33m{}\x1b[0m`", code);
        exit(1);
    }


    fn check(path : Option<RelativePathBuf>) {

        attempt!{
            start;
            "Preparing";
            reset()
        };

        let path = path.unwrap_or_else(|| RelativePathBuf::absolute_from(".")).join("main");

        attempt!{
            "Parsing";
            get_all_modules(None, path)
        };

        attempt!{
            "Checking";
            ProgramInfo::get().check_modules()
        }

    }


    fn build(path : Option<RelativePathBuf>) {
        Cli::check(path);

        attempt!{
            end;
            "Building";
            push_error!(InternalError, Always, {
                None => {"Todo : Build"}
            })
        };
    }


    fn run(path : Option<RelativePathBuf>) {
        Cli::build(path);
    }

}



/// Print a title, run a function, and report any warnings and/or errors.
/// If any errors were emitted, exit the program.
macro attempt {
    {$title:expr; $expr:expr} => {$crate::cli::attempt!{false, false; $title; $expr}},
    {start; $title:expr; $expr:expr} => {$crate::cli::attempt!{true, false; $title; $expr}},
    {end; $title:expr; $expr:expr} => {$crate::cli::attempt!{false, true; $title; $expr}},
    {start, end; $title:expr; $expr:expr} => {$crate::cli::attempt!{true, true; $title; $expr}},
    {$start:ident, $end:ident; $title:expr; $expr:expr} => {{
        if (! $start) {$crate::cli::printw!("\n");}
        $crate::cli::printw!(" \x1b[37m\x1b[2m=>\x1b[0m \x1b[96m{}\x1b[0m\x1b[36m\x1b[2m...\x1b[0m", $title);
        let v = $expr;
        match ($crate::notes::dump(4 + $title.len() + 13, $end)) {
            Ok(text) => {
                $crate::cli::printw!(" [\x1b[32m\x1b[1mSUCCESS\x1b[0m]\n");
                $crate::cli::printw!("{}", text);
            },
            Err(text) => {
                $crate::cli::printw!(" [\x1b[31m\x1b[1mFAILURE\x1b[0m]\n");
                $crate::cli::printw!("{}", text);
                $crate::cli::exit(1);
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
