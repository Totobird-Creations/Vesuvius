use clap::{
    Parser,
    Subcommand
};


pub mod colours {
    pub const PRIMARY   : &'static str = "1";
    pub const SECONDARY : &'static str = "5";
    pub const TERTIARY  : &'static str = "3";
}
pub mod info {
    pub const HOME    : &'static str = "https://github.com/Totobird-Creations/Vesuvius/";
    pub const LICENSE : &'static str = "GNU Lesser General Public License v2.1";
}


#[derive(Parser)]
pub(crate) struct Cli {

    #[command(subcommand)]
    command : Option<Command>

}


#[derive(Subcommand)]
enum Command {

    /// Show the documentation for a certain error.
    Explain {
        /// The error code.
        code : String
    }

}



impl Cli {

    pub(crate) fn run(self) {
        use Command::*;
        match (self.command) {
            None                 => {Cli::info()},
            Some(Explain {code}) => {Cli::explain(code)}
        }
    }

    fn info() {
        use colours::*;
        use info::*;
        let underline = "_".repeat(LICENSE.len());
        println!("
   \x1b[3{SECONDARY}m/^\\\x1b[0m  \x1b[9{TERTIARY}m{} \x1b[2mv\x1b[0m\x1b[9{TERTIARY}m\x1b[1m{}\x1b[0m
  \x1b[3{SECONDARY}m/   \\_\x1b[0m  \x1b[3{TERTIARY}m{HOME}\x1b[0m
 \x1b[3{SECONDARY}m/\x1b[9{PRIMARY}m\\  /  \x1b[3{SECONDARY}m\\\x1b[0m   \x1b[3{TERTIARY}m\x1b[2m{LICENSE}\x1b[0m
\x1b[3{SECONDARY}m/  \x1b[9{PRIMARY}m\\/\x1b[3{PRIMARY}mesuvius\x1b[3{SECONDARY}m{underline}\x1b[0m
",
        {
            let name = env!("CARGO_PKG_NAME").to_lowercase();
            name[0..1].to_uppercase() + &name[1..]
        },
        env!("CARGO_PKG_VERSION")
    );
    }

    fn explain(code : String) {
        todo!("Explain")
    }

}
