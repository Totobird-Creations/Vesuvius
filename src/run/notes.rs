use static_init::dynamic;


#[dynamic]
pub(crate) static mut COMPILATION_NOTES : Vec<CompilationNote> = Vec::new();


pub fn dump() {
    let lock = COMPILATION_NOTES.read();
    let mut counts = (
        0, // Warn
        0  // Error
    );
    for note in lock.iter() {
        println!("{}", note.fmt(&mut counts));
    }
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
        println!("{}", finished);
    }
}


macro_rules! push_error {
    ($typ:ident, $occur:ident, $($text:tt)*) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            level     : NoteType::Error(ErrorType::$typ),
            text      : format!($($text)*)
        };
        lock.push(note);
    }}
}
pub(crate) use push_error;

macro_rules! push_warn {
    ($typ:ident, $occur:ident, $($text:tt)*) => {{
        use $crate::run::notes::*;
        let mut lock = COMPILATION_NOTES.write();
        let     note = CompilationNote {
            occurance : NoteOccurance::$occur,
            level     : NoteType::Warn(WarnType::$typ),
            text      : format!($($text)*)
        };
        lock.push(note);
    }}
}
pub(crate) use push_warn;


pub struct CompilationNote {
    pub occurance : NoteOccurance,
    pub level     : NoteType,
    pub text      : String
}
impl CompilationNote {
    fn fmt(&self, counts : &mut (u64, u64)) -> String {
        return format!("{}", self.level.fmt(&self.occurance, counts));
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
    fn fmt(&self, occurance : &NoteOccurance, (warns, errors) : &mut (u64, u64)) -> String {
        return format!("{}[{}]\x1b[0m: {}\x1b[1m{}\x1b[0m.",
            self.cl(), self.pf(), self.cl(),
            match (self) {
                Self::Warn(warn) => {
                    *warns += 1;
                    warn.fmt(occurance)
                },
                Self::Error(error) => {
                    *errors += 1;
                    error.fmt(occurance)
                }
            }
        );
    }
}

enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

enum_named!{WarnType {
    UnstableReleaseUsed,
    BlockContents_Called
}}

enum_named!{ErrorType {
    DivisionByZero_
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
