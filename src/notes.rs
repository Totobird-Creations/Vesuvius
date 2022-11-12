//! Handles all of the warning and error queueing and printing.


use std::cmp::max;

use crate::{
    parse::node::Range,
    scope::ProgramInfo
};


/// Number of lines at the top and bottom of a
/// code snippet that should be shown before being
/// cut off and replaced with an ellipsis
const VERTICAL_CUTOFF : usize = 3;


/// Storage for all dumped and queued notes.
pub(crate) mod global {
    #![allow(non_camel_case_types)]
    use static_init::dynamic;
    use super::CompilationNote;

    /// All of the dumped notes. These are just used to
    /// display the final note count at the end of execution.
    #[dynamic]
    pub(crate) static mut COMPILATION_NOTES_DUMPED : Vec<CompilationNote> = Vec::new();

    /// All of the queued notes. These will be dumped once
    /// the compilation step is finished.
    #[dynamic]
    pub(crate) static mut COMPILATION_NOTES : Vec<CompilationNote> = Vec::new();

}


// Different occurance states, with the formatting functions auto generated..
enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

// Different error types, with the formatting functions auto generated.
enum_named!{ErrorType {
    /// Something within the compiler did not function properly, and so
    /// it was forced to crash.
    /// 
    /// If this occurs, please report it on the
    /// bug tracker.
    InternalError,
    /// When trying to import a library or module, the file could not
    /// be loaded.
    /// 
    /// Possible reasons:
    /// 
    /// - The file doesn't exist.
    /// 
    /// - You don't have permissions to the file.
    ModuleNotFound,
    /// While parsing, a character that wasn't expected was found.
    UnexpectedToken,
    /// Multiple `#[entry]` headers have been defined. The program can
    /// only start in one place, not multiple.
    DuplicateEntryHeader,
    /// An unexpected type was passed into a function, variable, etc.
    /// 
    /// Examples:
    /// 
    /// - If conditions must have a bool passed in.
    /// 
    /// - Variables, unless reinitialised, must stay the same type.
    InvalidTypeReceived,
    /// A non-existent symbol was attempted to be accessed.
    UnknownSymbol,
    /// A value was attempted to be modified, but it crossed either the min or max value.
    Bound_Broken
}}

// Different warning types, with the formatting functions auto generated.
enum_named!{WarnType += ErrorType {
    /// You are using a build of Vesuvius that isn't an official release.
    /// 
    /// It might be unstable and contain bugs.
    UnstableVersion,
    /// The contents of the given block are either always or never called.
    /// 
    /// This will usually show up in if statements, if the condition is always or never true.
    BlockContents_Called
}}


/// Get documentation for a certain note code.
pub(crate) fn explain(id : usize) -> Option<String> {
    try_explain!(ErrorType, id, Error);
    try_explain!(WarnType, id, Warn);
    return None;
}
macro try_explain {
    ($enm:ident, $id:expr, $typ:ident) => {
        if ($id < $enm::MAX) {
            let var  = $enm::from_id($id).unwrap();
            let doc  = var.doc()
                .unwrap_or_else(|| String::from("\x1b[37m\x1b[2m\x1b[3mNo documentation found for this note code.\x1b[0m"))
                .split("\n").map(|x| String::from("   ") + x)
                .collect::<Vec<_>>().join("\n");
            let note = NoteType::$typ(var);
            return Some(format!("{}\x1b[0m\n{}",
                note.title(None),
                doc
            ));
        }
    }
}


/// Dump all of the queued notes into a string, and remove them from the queue.
pub(crate) fn dump(mut line_len : usize, finish : bool) -> Result<String, String> {
    let mut final_text = String::new();

    let mut notes        = global::COMPILATION_NOTES.write();
    let mut notes_dumped = global::COMPILATION_NOTES_DUMPED.write();
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


/// Add a note to the queue, which will be dumped
/// after the compilation step is complete. Used by
/// `push_error!` and `push_warn!`.
#[allow(unused)]
macro _push_note {
    ($typ:expr, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        use $crate::notes::*;
        let mut lock = global::COMPILATION_NOTES.write();
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

/// Add an error to the queue which will be dumped
/// after the compilation step is complete.
/// 
/// Errors will kill the program, preventing the
/// next compilation step from executing.
#[allow(unused)]
pub macro push_error {
    ($typ:ident, $occur:ident) => {$crate::notes::push_error!($typ, $occur, {})},
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        $crate::notes::_push_note!(NoteType::Error(ErrorType::$typ), $occur, {$($range => {$($text)+}),*});
    }}
}

/// Add a warning to the queue which will be dumped
/// after the compilation step is complete.
/// 
/// Warnings will not prevent the next compilation
/// step from executing, but it will correct it.
#[allow(unused)]
pub macro push_warn {
    ($typ:ident, $occur:ident) => {$crate::notes::push_warn!($typ, $occur, {})},
    ($typ:ident, $occur:ident, {$($range:expr => {$($text:tt)+}),*}) => {{
        $crate::notes::_push_note!(NoteType::Warn(WarnType::$typ), $occur, {$($range => {$($text)+}),*});
    }}
}


/// A note. Stores where the note was created, the note type, and the details.
pub(crate) struct CompilationNote {
    /// Where the note was created (internally).
    /// Will be `None` in release builds as it is unneeded.
    source    : Option<(u32, u32, String)>,
    /// Whether the issue that is being pointed out will always, sometimes, or never happen.
    occurance : NoteOccurance,
    /// The note type. This is the title and a short, generic name.
    note      : NoteType,
    /// Details about the note.
    /// This includes:
    /// - The offending location that triggered this note.
    /// - Why there was a problem.
    /// - Potential fixes.
    details   : Vec<(Option<Range>, String)>
}
impl CompilationNote {
    /// Format the note, which can be printed when the note queue is dumped.
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

/// The notes types.
pub(crate) enum NoteType {
    /// User did something that is unrecommended.
    /// See `push_warn!` for more info.
    Warn(WarnType),
    /// User did something that is forbidden, or an internal error occured.
    /// See `push_error!` for more info.
    Error(ErrorType)
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
    // Get the formatted note title.
    fn title(&self, occurance : Option<&NoteOccurance>) -> String {
        // Get the note type info.
        let (title, id, id_len) = match (self) {
            Self::Warn(warn) => {
                (warn.fmt(occurance), warn.id(), warn.id_len())
            },
            Self::Error(error) => {
                (error.fmt(occurance), error.id(), error.id_len())
            }
        };
        // Get the formatted note title.
        return format!(" {}[ {}\x1b[2m(\x1b[0m{}{}\x1b[1m{}\x1b[0m{}\x1b[2m)\x1b[0m {}]\x1b[0m : {}\x1b[1m{}\x1b[0m.",
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
    }
    // Format the note.
    fn fmt(&self, occurance : &NoteOccurance, details : &Vec<(Option<Range>, String)>, (warns, errors) : &mut (u64, u64)) -> (String, usize) {
        // Get the note type info.
        let (title, id_len, internal_error) = match (self) {
            Self::Warn(warn) => {
                *warns += 1;
                (warn.fmt(Some(occurance)), warn.id_len(), false)
            },
            Self::Error(error) => {
                *errors += 1;
                (error.fmt(Some(occurance)), error.id_len(), matches!(error, ErrorType::InternalError))
            }
        };
        // Get the unformatted note title and get the length. This is used to make the note separators the correct length.
        let title_len = format!(" [ {}({}) ] : {}.",
            self.pf(),
            " ".repeat(id_len),
            title
        ).len();
        // Get the entire note.
        let text = format!("{}{}{}",
            self.title(Some(occurance)),
            if (details.len() > 0) {
                // Add details.
                details.iter().map(|detail| {
                    let (location, above_line, lines, below_line, message_prefix) = if let Some(range) = &detail.0 {
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
                            // Locaion
                            format!("   \x1b[90m┌\x1b[0m `\x1b[94m{:?}\x1b[0m` \x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m..\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m\x1b[34m:\x1b[0m\x1b[94m\x1b[1m{}\x1b[0m",
                                range.0,
                                range.1.0,
                                range.1.1,
                                range.2.0,
                                range.2.1,
                            ),
                            // Above Line
                            if (lines.len() > 1) {
                                format!("\n   \x1b[90m│ {} │\x1b[0m\x1b[95m\x1b[1m{}┌{}\x1b[0m",
                                    " ".repeat(lines_pad),
                                    " ".repeat(range.1.1),
                                    "─".repeat(lines[0].1.len() - range.1.1)
                                )
                            } else {String::new()},
                            // Lines
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
                            // Below Line
                            format!("\n   \x1b[90m│ {} │\x1b[0m\x1b[95m\x1b[1m{}\x1b[0m\n",
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
                            // Message Prefix
                            format!("└─{}─┴──", "─".repeat(lines_pad))
                        )
                    } else {
                        (
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::from("╶──────")
                        )
                    };
                    // Format the detail.
                    format!("\n{}{}{}{}{}",
                        // Location of the detail.
                        location,
                        // If the detail is multi line, add a marker showing the start of the detail, with a line to eol.
                        above_line,
                        // Add the code line.
                        lines,
                        // If the detail is single line, add a marker showing the start and end of the detail.
                        // If the detail is multi line, add a marker showing the end of the detail, with a line from the sol.
                        below_line,
                        // The detail message.
                        format!("   \x1b[90m{}\x1b[0m {}{}\x1b[0m", message_prefix, self.cl(), detail.1)
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



/// Auto generate functions for accessing indexes, names, and documentation of an enum variant.
macro_rules! enum_named {
    {$name:ident {$($(#[doc = $doc:literal])* $variant:ident),*}} => {
        $crate::notes::enum_named!{$name /=/ 0 {$($(#[doc = $doc])* $variant),*}}
    };
    {$name:ident += $addto:ident {$($(#[doc = $doc:literal])* $variant:ident),*}} => {
        $crate::notes::enum_named!{$name /=/ $addto::MAX {$($(#[doc = $doc])* $variant),*}}
    };
    {$name:ident /=/ $($addto:tt)::+ {$($(#[doc = $doc:literal])* $variant:ident),*}} => {
        /// An auto-generated enum.
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub enum $name {$(
            $(#[doc = $doc])?
            $variant
        ),*}
        #[allow(unused)]
        impl $name {
            /// One higher than the id of the last variant in this enum.
            const MAX : usize = {
                let mut i = $($addto)::+;
                $({Self::$variant}; i += 1;)*
                i
            };
            /// The id of the variant.
            fn id(&self) -> String {
                let mut i = $($addto)::+;
                $(if let Self::$variant = self {return format!("{:X}", i);} else {i += 1;})*
                panic!("INTERNAL ERROR");
            }
            /// The number of 0 to pad the id by.
            fn id_len(&self) -> usize {
                return max((Self::MAX - 1).to_string().len(), 4);
            }
            /// Get the variant from the code
            fn from_id(id : usize) -> Option<Self> {
                let mut i = $($addto)::+;
                $(if (id == i) {return Some(Self::$variant)} else {i += 1;})*
                return None;
            }
            /// The variant name, as it was given in the declaration.
            fn name<'l>(&self) -> &'l str {
                return match (self) {
                    $(Self::$variant => {stringify!($variant)}),*
                };
            }
            /// The doc of the variant.
            fn doc(&self) -> Option<String> {
                let vec : Vec<&str> = match (self) {
                    $(Self::$variant => vec![$($doc),*]),*
                };
                return if (vec.len() > 0) {
                    Some(vec.iter().map(|mut x| {let x = x.trim(); if (x.is_empty()) {String::from("\n")} else {format!("{x} ")}}).collect::<Vec<_>>().join(""))
                } else {None};
            }
            /// Formats the variant name, adding an occurance state name into it.
            /// 
            /// Steps:
            /// - Replace `_` with a space and the occurance state.
            /// - Insert space before uppercase letters.
            /// - Replace uppercase letters with their lowercase counterpart.
            /// - Trim spaces off of the edges of the string.
            /// - Replace first letter with uppercase counterpart.
            fn fmt(&self, occurance : Option<&NoteOccurance>) -> String {
                return match (self) {
                    $(Self::$variant => {
                        let mut string = String::new();
                        for ch in stringify!($variant).chars() {
                            if (ch == '_') {
                                if let Some(occurance) = occurance {
                                    string.push(' ');
                                    string.push_str(occurance.name());
                                }
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
