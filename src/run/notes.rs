use std::fmt::{
    Display,
    Formatter,
    Result
};

pub struct CompilationNote {
    pub occurance : NoteOccurance,
    pub level     : NoteType,
    pub text      : String
}
impl Display for CompilationNote {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", self.level.fmt(&self.occurance));
    }
}

pub enum NoteType {
    Warn(WarnType),  // User did something that is unrecommended.
    Error(ErrorType) // User did something that is forbidden.
}
impl NoteType {
    fn fmt(&self, occurance : &NoteOccurance) -> String {
        return format!("{}", match (self) {
            Self::Warn  (warn)  => warn.fmt(occurance),
            Self::Error (error) => error.fmt(occurance)
        });
    }
}

enum_named!{NoteOccurance {
    Always,
    Sometimes,
    Never
}}

enum_named!{WarnType {
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
