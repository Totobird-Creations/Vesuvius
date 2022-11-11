use std::cmp::max;

use static_init::dynamic;

use crate::{
    parse::node::Range,
    verify::scope::ProgramInfo
};


const VERTICAL_CUTOFF : usize = 3;

#[dynamic]
pub(crate) static mut COMPILATION_NOTES_DUMPED : Vec<CompilationNote> = Vec::new();
#[dynamic]
pub(crate) static mut COMPILATION_NOTES : Vec<CompilationNote> = Vec::new();


// Different occurance states, with the formatting functions auto generated..
enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

// Different error types, with the formatting functions auto generated.
enum_named!{ErrorType {
    #[doc("Something within the compiler did not function properly.")]
    InternalError,
    FileNotFound,
    UnexpectedToken,
    DuplicateEntryHeader,
    InvalidTypeReceived,
    UnknownSymbol,
    Bound_Broken
}}

// Different warning types, with the formatting functions auto generated.
enum_named!{WarnType += ErrorType {
    UnstableVersion,
    BlockContents_Called
}}


pub fn explain(_id : usize) {
    todo!();
}


pub fn dump(mut line_len : usize, finish : bool) -> Result<String, String> {
    let mut final_text = String::new();

    let mut notes        = COMPILATION_NOTES.write();
    let mut notes_dumped = COMPILATION_NOTES_DUMPED.write();
    let mut counts = (
        0, // Warn
        0  // Error
    );
    // For each note, print a line, and the note.
    for note in notes.iter() {
        let res = note.fmt(&mut counts);
        final_text += &format!("\x1b[90m{}\x1b[0m\n{}\n", "─".repeat(max(res.1, line_len)), res.0);
        line_len = res.1;
    }
    for note_dumped in notes_dumped.iter() {
        match (note_dumped.note) {
            NoteType::Warn  (_) => {counts.0 += 1},
            NoteType::Error (_) => {counts.0 += 1}
        }
    }
    // Print a line after the last note.
    final_text += &format!("\x1b[90m{}\x1b[0m\n", "─".repeat(line_len));
    // Print finished or failed, with the number of each note type.
    let (warns, errors) = counts;
    if (finish || errors > 0) {
        // Finished or failed.
        let (mut finished, mut finished_len) = if (errors > 0) {
            (String::from("\x1b[31m\x1b[1mFailed\x1b[0m"), 6)
        } else {
            (String::from("\x1b[32m\x1b[1mFinished\x1b[0m"), 8)
        };
        // Note type counts.
        let mut with = Vec::new();
        if (warns > 0)  {
            let message   = format!("{} warning{}", warns, if (warns != 1) {"s"} else {""});
            finished_len += message.len();
            with.push(format!("\x1b[33m{}\x1b[0m", message));
        }
        if (errors > 0) {
            let message   = format!("{} error{}", warns, if (warns != 1) {"s"} else {""});
            finished_len += message.len();
            with.push(format!("\x1b[31m{}\x1b[0m", message));
        }
        if (with.len() > 0) {
            finished     += " with ";
            finished_len += 6;
            finished     += &with[0];
            if (with.len() > 2) {
                for section in &with[1..(with.len() - 1)] {
                    finished     += ", ";
                    finished_len += 2;
                    finished     += section;
                }
            }
            if (with.len() > 1) {
                finished     += " and ";
                finished_len += 5;
                finished     += &with[with.len() - 1];
            }
        }
        // Print final message.
        final_text += &format!("\n \x1b[37m\x1b[2m=>\x1b[0m {}.\n", finished);
        final_text += &format!("\x1b[90m{}\x1b[0m\n\n", "─".repeat(5 + finished_len));
    }
    notes_dumped.append(&mut notes);
    return if (errors > 0) {
        Err(final_text)
    } else {
        Ok(final_text)
    };
}


// Add a note to be printed after verification is complete.
#[allow(unused)]
macro _push_note {
    ($typ:expr, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            source : if (cfg!(debug_assertions)) {
                // If in debug env, Get the location of the call.
                Some((line!(), column!(), String::from(module_path!())))
            } else {None},
            occurance : NoteOccurance::$occur,
            note      : $typ,
            details   : vec![$(($range, format!($($text)+))),*]
        };
        lock.push(note);
    }}
}

// Add an error to be printed after verification is complete.
// Errors will prevent compilation.
#[allow(unused)]
pub macro push_error {
    ($typ:ident, $occur:ident) => {$crate::notes::push_error!($typ, $occur, {})},
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        $crate::notes::_push_note!(NoteType::Error(ErrorType::$typ), $occur, {$($range => {$($text)+}),*});
    }}
}

// Add a warning to be printed after verification is complete.
// Warnings will not prevent compilation, but will be corrected by the verifier.
#[allow(unused)]
pub macro push_warn {
    ($typ:ident, $occur:ident) => {$crate::notes::push_warn!($typ, $occur, {})},
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        $crate::notes::_push_note!(NoteType::Warn(WarnType::$typ), $occur, {$($range => {$($text)+}),*});
    }}
}


pub struct CompilationNote {
    pub source    : Option<(u32, u32, String)>,
    pub occurance : NoteOccurance,
    pub note      : NoteType,
    pub details   : Vec<(Option<Range>, String)>
}
impl CompilationNote {
    fn fmt(&self, counts : &mut (u64, u64)) -> (String, usize) {
        let text = self.note.fmt(&self.occurance, &self.details, counts);
        return (
            format!("{}{}",
                if let Some((line, col, file)) = &self.source {
                    format!(" \x1b[37m\x1b[2m\x1b[3mInternal source:\x1b[0m \x1b[37m\x1b[2m`\x1b[0m\x1b[37m\x1b[1m{}\x1b[1m\x1b[0m\x1b[37m\x1b[2m`\x1b[0m \x1b[37m\x1b[1m{}\x1b[0m\x1b[97m\x1b[2m:\x1b[0m\x1b[37m\x1b[1m{}\x1b[0m\n",
                        file,
                        line, col
                    )
                } else {String::new()},
                text.0
            ),
            text.1
        );
    }
}

pub enum NoteType {
    Warn(WarnType),  // User did something that is unrecommended.
    Error(ErrorType) // User did something that is forbidden, or internal error.
}
impl NoteType {
    // Return the ansi escape colour code of this note type.
    fn cl(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "\x1b[33m",
            Self::Error (_) => "\x1b[31m"
        }
    }
    // Get the brightened ansi escape colour code of this note type.
    fn clp(&self) -> &'static str {
        return match (self) {
            Self::Warn  (_) => "\x1b[93m",
            Self::Error (_) => "\x1b[91m"
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
    fn fmt(&self, occurance : &NoteOccurance, details : &Vec<(Option<Range>, String)>, (warns, errors) : &mut (u64, u64)) -> (String, usize) {
        // Get the note type info.
        let (title, id, id_len, internal_error) = match (self) {
            Self::Warn(warn) => {
                *warns += 1;
                (warn.fmt(occurance), warn.id(), warn.id_len(), false)
            },
            Self::Error(error) => {
                *errors += 1;
                (error.fmt(occurance), error.id(), error.id_len(), matches!(error, ErrorType::InternalError))
            }
        };
        // Get the unformatted note title and get the length. This is used to make the note separators the correct length.
        let title_len = format!(" [ {}({}) ] : {}.",
            self.pf(),
            " ".repeat(id_len),
            title
        ).len();
        // Get the formatted note title.
        let title = format!(" {}[ {}\x1b[2m(\x1b[0m{}{}\x1b[1m{}\x1b[0m{}\x1b[2m)\x1b[0m {}]\x1b[0m : {}\x1b[1m{}\x1b[0m.",
            self.cl(),
            self.pf(),
            self.clp(),
            "0".repeat(id_len - if (id == "0") {0} else {id.len()}),
            if (id == "0") {String::new()} else {id},
            self.cl(),
            self.cl(),
            self.clp(),
            title
        );
        // Get the entire note.
        let text = format!("{}{}{}",
            title,
            if (details.len() > 0) {
                // Add details.
                details.iter().map(|detail| {
                    let (location, above_line, lines, below_line, lines_pad) = if let Some(range) = &detail.0 {
                        // Get the script at the file of the detail.
                        let script = ProgramInfo::get().script_of(&range.0);
                        // Get the location of the detail.
                        let range = range.to_linecolumn(&script);
                        // Get the lines, and cut off unneeded information.
                        let mut lines       = Vec::new();
                        let mut lines_pad   = 0;
                        let mut vert_cutoff = None;
                        for l in (range.1.0 - 1)..range.2.0 {
                            if (l - (range.1.0 - 1) >= VERTICAL_CUTOFF && range.2.0 - l > VERTICAL_CUTOFF) {
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
                        (
                            format!("   \x1b[90m┌\x1b[0m `\x1b[94m{:?}\x1b[0m` \x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m..\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m",
                                range.0,
                                range.1.0,
                                range.1.1,
                                range.2.0,
                                range.2.1,
                            ),
                            if (lines.len() > 1) {
                                format!("\n   \x1b[90m│ {} │\x1b[0m\x1b[95m\x1b[1m{}┌{}\x1b[0m",
                                    " ".repeat(lines_pad),
                                    " ".repeat(range.1.1),
                                    "─".repeat(lines[0].1.len() - range.1.1)
                                )
                            } else {String::new()},
                            lines.iter().map(|(l, line)| {
                                format!("\n   \x1b[90m│\x1b[0m \x1b[94m\x1b[2m{: >lines_pad$}\x1b[0m \x1b[90m│\x1b[0m {}{}",
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
                            format!("\n   \x1b[90m│ {} │\x1b[0m\x1b[95m\x1b[1m{}\x1b[0m",
                                " ".repeat(lines_pad),
                                // If the detail is single line, add a marker showing the start and end of the detail.
                                if (lines.len() <= 1) {
                                    format!("{}{}",
                                        " ".repeat(range.1.1),
                                        if (range.2.1 - range.1.1 <= 1) {
                                            String::from("╵")
                                        } else if (range.1.1 != range.2.1) {
                                            format!("└{}┘", "─".repeat(range.2.1 - range.1.1 - 2))
                                        } else {String::new()}
                                    )
                                // If the detail is multi line, add a marker showing the end of the detail, with a line from the sol.
                                } else {format!(" {}┘", "─".repeat(range.2.1 - 2))}
                            ),
                            lines_pad
                        )
                    } else {
                        (
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            0
                        )
                    };
                    // Format the detail.
                    format!("\n{}{}{}{}\n{}",
                        // Location of the detail.
                        location,
                        // If the detail is multi line, add a marker showing the start of the detail, with a line to eol.
                        above_line,
                        // Add the code line.
                        lines,
                        below_line,
                        // The detail message.
                        format!("   \x1b[90m└─{}─┴──\x1b[0m {}{}\x1b[0m", "─".repeat(lines_pad), self.cl(), detail.1)
                    )
                }).collect::<Vec<_>>().join("")
            } else {
                // If no details were provided, mention it.
                String::from("\n \x1b[37m\x1b[2m\x1b[3mNo other details provided.\x1b[0m")
            },
            if (internal_error) {
                if (cfg!(debug_assertions)) {
                    format!("\n \x1b[37m\x1b[2m\x1b[3mThis is a debug build of {}.\n Do not report this on the bug tracker.\x1b[0m", env!("CARGO_PKG_NAME"))
                } else {
                    String::from("\n \x1b[37m\x1b[2m\x1b[3mPlease report this at\x1b[0m: \x1b[37m\x1b[2m`\x1b[1mhttps://github.com/Totobird-Creations/Vesuvius/issues/\x1b[0m\x1b[37m\x1b[2m`\x1b[0m.")
                }
            } else {
                String::new()
            }
        );
        return (text, title_len);
    }
}



// Auto generate functions for formatting a note type, with an occurance state.
macro_rules! enum_named {
    {$name:ident {$($(#[doc($($doc:literal),*)])? $variant:ident),*}} => {
        $crate::notes::enum_named!{$name /=/ 0 {$($(#[doc($($doc),*)])? $variant),*}}
    };
    {$name:ident += $addto:ident {$($(#[doc($($doc:literal),*)])? $variant:ident),*}} => {
        $crate::notes::enum_named!{$name /=/ $addto::MAX {$($(#[doc($($doc),*)])? $variant),*}}
    };
    {$name:ident /=/ $($addto:tt)::+ {$($(#[doc($($doc:literal),*)])? $variant:ident),*}} => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant),*
        }
        #[allow(unused)]
        impl $name {
            // One higher than the id of the last variant in this enum.
            const MAX : u32 = {
                let mut i = $($addto)::+;
                $({Self::$variant}; i += 1;)*
                i
            };
            // The id of the variant.
            fn id(&self) -> String {
                let mut i = $($addto)::+;
                $(if let Self::$variant = self {return format!("{:X}", i);} else {i += 1;})*
                panic!("INTERNAL ERROR");
            }
            // The number of 0 to pad the id by.
            fn id_len(&self) -> usize {
                return max((Self::MAX - 1).to_string().len(), 4);
            }
            // The variant name, as it was given in the declaration.
            fn name<'l>(&self) -> &'l str {
                return match (self) {
                    $(Self::$variant => {stringify!($variant)}),*
                };
            }
            // The doc of the variant.
            fn doc(&self) -> Vec<String> {
                return match (self) {
                    $(Self::$variant => vec![$($(String::from($doc)),*)?]),*
                }
            }
            // Steps:
            // - Replace `_` with a space and the occurance state.
            // - Insert space before uppercase letters.
            // - Replace uppercase letters with their lowercase counterpart.
            // - Trim spaces off of the edges of the string.
            // - Replace first letter with uppercase counterpart.
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
    };
}
use enum_named;
