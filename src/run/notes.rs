use std::cmp::max;

use static_init::dynamic;

use crate::parse::node::Range;


const DETAILS_SNIPPET_CUTOFF : usize = 1;


#[dynamic]
pub(crate) static mut COMPILATION_NOTES : Vec<CompilationNote> = Vec::new();


pub fn dump<'l, S : Into<&'l String>>(script : S) {
    let lock = COMPILATION_NOTES.read();
    let mut counts = (
        0, // Warn
        0  // Error
    );
    let mut line_len = 0;
    let     script   = script.into();
    for note in lock.iter() {
        let res  = note.fmt(script, &mut counts);
        println!("\x1b[90m{}\x1b[0m\n{}", "─".repeat(max(res.1, line_len)), res.0);
        line_len = res.1;
    }
    println!("\x1b[90m{}\x1b[0m", "─".repeat(line_len));
    {
        let (warns, errors) = counts;
        let mut finished = if (errors > 0) {
            String::from("\x1b[31m\x1b[1mFailed\x1b[0m")
        } else {
            String::from("\x1b[32m\x1b[1mFinished\x1b[0m")
        };
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
        println!("\n{}", finished);
    }
}


#[allow(unused)]
macro_rules! push_error {
    ($typ:ident, $occur:ident) => {$crate::run::notes::push_error!($typ, $occur, {})};
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            level     : NoteType::Error(ErrorType::$typ),
            details   : vec![$(($range, format!($($text)+))),*]
        };
        lock.push(note);
    }}
}
pub(crate) use push_error;

#[allow(unused)]
macro_rules! push_warn {
    ($typ:ident, $occur:ident) => {$crate::run::notes::push_warn!($typ, $occur, {})};
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            level     : NoteType::Warn(WarnType::$typ),
            details   : vec![$(($range, format!($($text)+))),*]
        };
        lock.push(note);
    }}
}
pub(crate) use push_warn;


pub struct CompilationNote {
    pub occurance : NoteOccurance,
    pub level     : NoteType,
    pub details   : Vec<(Range, String)>
}
impl CompilationNote {
    fn fmt(&self, script : &String, counts : &mut (u64, u64)) -> (String, usize) {
        let text = self.level.fmt(script, &self.occurance, &self.details, counts);
        return (format!("{}", text.0), text.1);
    }
}

pub enum NoteType {
    Warn(WarnType),  // User did something that is unrecommended.
    Error(ErrorType) // User did something that is forbidden.
}
impl NoteType {
    fn cl(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "\x1b[33m",
            Self::Error (_) => "\x1b[31m"
        }
    }
    fn pf(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "WARN",
            Self::Error (_) => "ERROR"
        }
    }
    fn fmt(&self, script : &String, occurance : &NoteOccurance, details : &Vec<(Range, String)>, (warns, errors) : &mut (u64, u64)) -> (String, usize) {
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
        let title_unformat = format!("[{}]: {}.",
            self.pf(),
            title
        );
        let title = format!("{}[{}]\x1b[0m: {}\x1b[1m{}\x1b[0m.",
            self.cl(),
            self.pf(),
            self.cl(),
            title
        );
        let text = format!("{}{}",
            title,
            if (details.len() > 0) {
                details.iter().map(|detail| {
                    let     range      = detail.0.to_linecolumn(script);
                    let mut lines      = Vec::new();
                    let mut lines_pad  = 0;
                    for l in (range.0.0 - 1)..range.1.0 {
                        let line     = script.lines().nth(l).unwrap();
                        let line_pad = l.to_string().len();
                        if (line_pad > lines_pad) {
                            lines_pad = line_pad;
                        }
                        lines.push((l, line));
                    }
                    while (lines.len() > 4) {
                        lines.remove(2);
                    }
                    format!("{}{}{}{}\n{}",
                        format!("\n  \x1b[90m┌\x1b[0m `\x1b[94m{}\x1b[0m` \x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m..\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m",
                            "todo_filename.vsv",
                            range.0.0,
                            range.0.1,
                            range.1.0,
                            range.1.1
                        ),
                        if (lines.len() > 1) {
                            format!("\n  \x1b[90m│ {} │\x1b[0m\x1b[95m{}┌{}\x1b[0m",
                                " ".repeat(lines_pad),
                                " ".repeat(range.0.1),
                                "─".repeat(lines[0].1.len() - range.0.1)
                            )
                        } else {String::new()},
                        lines.iter().map(|(l, line)| {
                            format!("\n  \x1b[90m│\x1b[0m \x1b[94m\x1b[2m{: >lines_pad$}\x1b[0m \x1b[90m│\x1b[0m {}", l, line)
                        }).collect::<Vec<_>>().join(""),
                        format!("\n  \x1b[90m│ {} │\x1b[0m\x1b[95m{}\x1b[0m",
                            " ".repeat(lines_pad),
                            if (lines.len() <= 1) {
                                format!("{}{}",
                                    " ".repeat(range.0.1),
                                    if (range.1.1 - range.0.1 <= 1) {
                                        String::from("╵")
                                    } else if (range.0.1 != range.1.1) {
                                        format!("└{}┘", "─".repeat(range.1.1 - range.0.1 - 2))
                                    } else {String::new()}
                                )
                            } else {format!("{}┘", "─".repeat(range.1.1 - 1))}
                        ),
                        format!("  \x1b[90m└\x1b[0m {}{}\x1b[0m", self.cl(), detail.1)
                    )
                }).collect::<Vec<_>>().join("")
            } else {
                String::from("\n  \x1b[90m\x1b[3mNo other details provided.\x1b[0m")
            }
        );
        return (text, title_unformat.len());
    }
}

enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

enum_named!{WarnType {
    UnstableRelease,
    BlockContents_Called
}}

enum_named!{ErrorType {
    DuplicateEntry,
    InvalidTypeReceived
}}



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
