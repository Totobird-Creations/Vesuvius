use std::cmp::max;

use static_init::dynamic;

use crate::parse::node::Range;


const VERTICAL_CUTOFF : usize = 3;

#[dynamic]
pub(crate) static mut COMPILATION_NOTES : Vec<CompilationNote> = Vec::new();


// Different occurance states, with the formatting functions auto generated..
enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

// Different warning types, with the formatting functions auto generated.
enum_named!{WarnType {
    UnstableVersion,
    BlockContents_Called
}}

// Different error types, with the formatting functions auto generated.
enum_named!{ErrorType {
    DuplicateEntryHeader,
    InvalidTypeReceived,
    UnknownSymbol,
    _DividedByZero
}}


pub fn dump<'l, S : Into<&'l String>>(script : S) {
    let lock = COMPILATION_NOTES.read();
    let mut counts = (
        0, // Warn
        0  // Error
    );
    let mut line_len = 0;
    let     script   = script.into();
    // For each note, print a line, and the note.
    for note in lock.iter() {
        let res  = note.fmt(script, &mut counts);
        println!("\x1b[90m{}\x1b[0m\n{}", "─".repeat(max(res.1, line_len)), res.0);
        line_len = res.1;
    }
    // Print a line after the last note.
    println!("\x1b[90m{}\x1b[0m", "─".repeat(line_len));
    // Print finished or failed, with the number of each note type.
    {
        let (warns, errors) = counts;
        // Finished or failed.
        let mut finished = if (errors > 0) {
            String::from("\x1b[31m\x1b[1mFailed\x1b[0m")
        } else {
            String::from("\x1b[32m\x1b[1mFinished\x1b[0m")
        };
        // Note type counts.
        let mut with = Vec::new();
        if (warns > 0)  {with.push(format!("\x1b[33m{} warning{}\x1b[0m" , warns  , if (warns != 1)  {"s"} else {""}));}
        if (errors > 0) {with.push(format!("\x1b[31m{} error{}\x1b[0m"   , errors , if (errors != 1) {"s"} else {""}));}
        if (with.len() > 0) {
            finished += " with ";
            finished += &with[0];
            if (with.len() > 2) {
                for section in &with[1..(with.len() - 1)] {
                    finished += ", ";
                    finished += section;
                }
            }
            if (with.len() > 1) {
                finished += " and ";
                finished += &with[with.len() - 1];
            }
        }
        finished += ".";
        // Print final message.
        println!("\n{}", finished);
    }
}


// Add an error to be printed after verification is complete.
// Errors will prevent compilation.
#[allow(unused)]
macro_rules! push_error {
    ($typ:ident, $occur:ident) => {$crate::run::notes::push_error!($typ, $occur, {})};
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            note      : NoteType::Error(ErrorType::$typ),
            details   : vec![$(($range, format!($($text)+))),*]
        };
        lock.push(note);
    }}
}
pub(crate) use push_error;

// Add a warning to be printed after verification is complete.
// Warnings will not prevent compilation, but will be corrected by the verifier.
#[allow(unused)]
macro_rules! push_warn {
    ($typ:ident, $occur:ident) => {$crate::run::notes::push_warn!($typ, $occur, {})};
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            note      : NoteType::Warn(WarnType::$typ),
            details   : vec![$(($range, format!($($text)+))),*]
        };
        lock.push(note);
    }}
}
pub(crate) use push_warn;


pub struct CompilationNote {
    pub occurance : NoteOccurance,
    pub note      : NoteType,
    pub details   : Vec<(Range, String)>
}
impl CompilationNote {
    fn fmt(&self, script : &String, counts : &mut (u64, u64)) -> (String, usize) {
        let text = self.note.fmt(script, &self.occurance, &self.details, counts);
        return (format!("{}", text.0), text.1);
    }
}

pub enum NoteType {
    Warn(WarnType),  // User did something that is unrecommended.
    Error(ErrorType) // User did something that is forbidden.
}
impl NoteType {
    // Return the ansi escape colour code of this note level.
    fn cl(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "\x1b[33m",
            Self::Error (_) => "\x1b[31m"
        }
    }
    // Return the title of this note level.
    fn pf(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "WARN",
            Self::Error (_) => "ERROR"
        }
    }
    // Format the note.
    fn fmt(&self, script : &String, occurance : &NoteOccurance, details : &Vec<(Range, String)>, (warns, errors) : &mut (u64, u64)) -> (String, usize) {
        // Get the note type name.
        let title = match (self) {
            Self::Warn(warn) => {
                *warns += 1;
                warn.fmt(occurance)
            },
            Self::Error(error) => {
                *errors += 1;
                error.fmt(occurance)
            }
        };
        // Get the unformatted note title and get the length. This is used to make the note separators the correct length.
        let title_len = format!("[{}]: {}.",
            self.pf(),
            title
        ).len();
        // Get the formatted note title.
        let title = format!("{}[{}]\x1b[0m: {}\x1b[1m{}\x1b[0m.",
            self.cl(),
            self.pf(),
            self.cl(),
            title
        );
        // Get the entire note.
        let text = format!("{}{}",
            title,
            if (details.len() > 0) {
                // Add details.
                details.iter().map(|detail| {
                    // Get the location of the detail.
                    let     range       = detail.0.to_linecolumn(script);
                    // Get the lines, and cut off unneeded information.
                    let mut lines       = Vec::new();
                    let mut lines_pad   = 0;
                    let mut vert_cutoff = None;
                    for l in (range.0.0 - 1)..range.1.0 {
                        if (l - (range.0.0 - 1) >= VERTICAL_CUTOFF && range.1.0 - l > VERTICAL_CUTOFF) {
                            if (matches!(vert_cutoff, None)) {
                                vert_cutoff = Some(l);
                            }
                            continue;
                        }
                        let line     = script.lines().nth(l).unwrap();
                        let line_pad = (l + 1).to_string().len();
                        if (line_pad > lines_pad) {
                            lines_pad = line_pad;
                        }
                        lines.push((l + 1, line));
                    }
                    // Format the detail.
                    format!("\n{}{}{}{}\n{}",
                        // Location of the detail.
                        format!("  \x1b[90m┌\x1b[0m `\x1b[94m{}\x1b[0m` \x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m..\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m",
                            "todo_filename.vsv",
                            range.0.0,
                            range.0.1,
                            range.1.0,
                            range.1.1
                        ),
                        // If the detail is multi line, add a marker showing the start of the detail, with a line to eol.
                        if (lines.len() > 1) {
                            format!("\n  \x1b[90m│ {} │\x1b[0m\x1b[95m{}┌{}\x1b[0m",
                                " ".repeat(lines_pad),
                                " ".repeat(range.0.1),
                                "─".repeat(lines[0].1.len() - range.0.1)
                            )
                        } else {String::new()},
                        // Add the code line.
                        lines.iter().map(|(l, line)| {
                            format!("\n  \x1b[90m│\x1b[0m \x1b[94m\x1b[2m{: >lines_pad$}\x1b[0m \x1b[90m│\x1b[0m {}{}",
                                l, line,
                                // If some of the lines were cut off, add an empty line and some dots.
                                if let Some(vert_cutoff) = vert_cutoff {
                                    if (vert_cutoff == *l) {
                                        format!("\n  \x1b[90m│\x1b[0m \x1b[94m\x1b[2m{}\x1b[0m \x1b[90m│\x1b[0m",
                                            "·".repeat(lines_pad)
                                        )
                                    } else {String::new()}
                                } else {String::new()}
                            )
                        }).collect::<Vec<_>>().join(""),
                        format!("\n  \x1b[90m│ {} │\x1b[0m\x1b[95m{}\x1b[0m",
                            " ".repeat(lines_pad),
                            // If the detail is single line, add a marker showing the start and end of the detail.
                            if (lines.len() <= 1) {
                                format!("{}{}",
                                    " ".repeat(range.0.1),
                                    if (range.1.1 - range.0.1 <= 1) {
                                        String::from("╵")
                                    } else if (range.0.1 != range.1.1) {
                                        format!("└{}┘", "─".repeat(range.1.1 - range.0.1 - 2))
                                    } else {String::new()}
                                )
                            // If the detail is multi line, add a marker showing the end of the detail, with a line from the sol.
                            } else {format!(" {}┘", "─".repeat(range.1.1 - 2))}
                        ),
                        // The detail message.
                        format!("  \x1b[90m└─{}─┴──\x1b[0m {}{}\x1b[0m", "─".repeat(lines_pad), self.cl(), detail.1)
                    )
                }).collect::<Vec<_>>().join("")
            } else {
                // If no details were provided, mention it.
                String::from("\n  \x1b[90m\x1b[3mNo other details provided.\x1b[0m")
            }
        );
        return (text, title_len);
    }
}



// Auto generate functions for formatting a note type, with an occurance state.
macro_rules! enum_named {
    {$name:ident {$($variant:ident),*}} => {
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant),*
        }
        #[allow(unused)]
        impl $name {
            fn name<'l>(&self) -> &'l str {
                match (self) {
                    $(Self::$variant => {stringify!($variant)}),*
                }
            }
            fn fmt(&self, occurance : &NoteOccurance) -> String {
                return match (self) {
                    $(Self::$variant => {
                        let mut string = String::new();
                        for ch in stringify!($variant).chars() {
                            if (ch == '_') {
                                string.push(' ');
                                string.push_str(occurance.name());
                            } else {
                                if (ch.is_uppercase()) {
                                    string.push(' ');
                                }
                                string.push(ch);
                            }
                        }
                        string = string.to_lowercase().trim().to_owned();
                        string[0..1].to_uppercase() + &string[1..]
                    }),*
                };
            }
        }
    }
}
use enum_named;
