use std::fmt::{
    Display,
    Formatter,
    Result,
    Debug
};

use crate::parser::node::*;



const INDENT      : &'static str = "  ";

const HEADER      : Colour       = Colour( 124 , 166 , 104 , true  );
const KEYWORD     : Colour       = Colour( 197 , 134 , 192 , false );
const OBJECT      : Colour       = Colour( 86  , 156 , 214 , false );
const NAME        : Colour       = Colour( 220 , 220 , 170 , false );
//const LIT_STRING  : Colour       = Colour( 206 , 145 , 120 , false );
const LIT_NUMERIC : Colour       = Colour( 181 , 206 , 168 , false );



struct Colour(
    u8,  // Red
    u8,  // Green
    u8,  // Blue
    bool // Italic
);
impl Display for Colour {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "\x1b[38;2;{};{};{}m{}",
            self.0, self.1, self.2,
            if (self.3) {"\x1b[3m"} else {""}
        );
    }
}



impl Display for Program {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", self.decls.iter().map(|decl| format!("{};", decl.format(0))).collect::<Vec<String>>().join("\n"));
    }
}


impl Declaration {
    fn format(&self, indent : usize) -> String {
        return format!("{}{} {}",
            self.headers.iter()
                .map(|header| format!("{}\n{}",
                    header.format(indent),
                    INDENT.repeat(indent)
                ))
                .collect::<Vec<String>>()
                .join(""),
            self.vis.format(indent),
            self.decl.format(indent)
        );
    }
}


impl DeclarationHeader {
    pub fn format(&self, _indent : usize) -> String {
        return c!(HEADER, format!("#[{}]", match (self) {
            Self::Entry => "entry"
        }));
    }
}


impl DeclarationVisibility {
    pub fn format(&self, _indent : usize) -> String {
        return c!(KEYWORD, match (self) {
            Self::Public  => "pub",
            Self::Private => "priv"
        });
    }
}


impl DeclarationType {
    fn format(&self, indent : usize) -> String {
        return match (self) {

            Self::Function(name, _args, _ret, block) => {
                format!("{} {} {}",
                    c!(OBJECT, "fn"),
                    c!(NAME, name),
                    block.format(indent)
                )
            }

        };
    }
}


impl Statement {
    fn format(&self, indent : usize) -> String {
        return match (self) {

            Self::InitVar(name, value) => {
                format!("{} {} = {}",
                    c!(KEYWORD, "let"),
                    c!(NAME, name),
                    value.format(indent)
                )
            },

            Self::Expression(expr) => expr.format(indent)

        }
    }
}


impl Expression {
    fn format(&self, indent : usize) -> String {
        return match (self) {

            Self::EqualsOperation         (left, right) => format!("({} == {})", left.format(indent), right.format(indent)),
            Self::NotEqualsOperation      (left, right) => format!("({} != {})", left.format(indent), right.format(indent)),
            Self::GreaterOperation        (left, right) => format!("({} > {})", left.format(indent), right.format(indent)),
            Self::GreaterEqualsOperation  (left, right) => format!("({} >= {})", left.format(indent), right.format(indent)),
            Self::LessOperation           (left, right) => format!("({} < {})", left.format(indent), right.format(indent)),
            Self::LessEqualsOperation     (left, right) => format!("({} <= {})", left.format(indent), right.format(indent)),
            Self::AdditionOperation       (left, right) => format!("({} + {})", left.format(indent), right.format(indent)),
            Self::SubtractionOperation    (left, right) => format!("({} - {})", left.format(indent), right.format(indent)),
            Self::MultiplicationOperation (left, right) => format!("({} * {})", left.format(indent), right.format(indent)),
            Self::DivisionOperation       (left, right) => format!("({} / {})", left.format(indent), right.format(indent)),

            Self::Atom(atom) => atom.format(indent)

        }
    }
}


impl Atom {
    fn format(&self, indent : usize) -> String {
        return match (self) {

            Self::Literal(lit) => lit.format(indent),

            Self::Expression(expr) => expr.format(indent),

            Self::If(ifs, els) => {
                format!("{}{}{}",
                    c!(KEYWORD, "if"),
                    ifs.iter()
                        .map(|i| format!(" ({}) {}",
                            i.0.format(indent),
                            i.1.format(indent)
                        ))
                        .collect::<Vec<String>>()
                        .join(&format!("\n{}{}",
                            INDENT.repeat(indent),
                            c!(KEYWORD, "elif")
                        )),
                    if let Some(els) = els {
                        format!("\n{}{} {}",
                            INDENT.repeat(indent),
                            c!(KEYWORD, "else"),
                            els.format(indent)
                        )
                    } else {
                        String::new()
                    }
                )
            }

        }
    }
}


impl Literal {
    fn format(&self, _indent : usize) -> String {
        return match (self) {

            Self::Int(int) => c!(LIT_NUMERIC, int),

            Self::Float(int, dec) => {
                format!("{}.{}",
                    c!(LIT_NUMERIC, int),
                    c!(LIT_NUMERIC, dec)
                )
            },

            Self::Identifier(name) => c!(NAME, name)

        }
    }
}


impl Block {
    fn format(&self, indent : usize) -> String {
        return format!("{{{}{}\n{}}}",
            self.stmts.iter()
                .map(|stmt| format!("\n{}{}",
                    INDENT.repeat(indent + 1),
                    stmt.format(indent + 1)
                ))
                .collect::<Vec<String>>()
                .join(";"),
            if (self.retlast) {""} else {";"},
            INDENT.repeat(indent)
        );
    }
}
impl Debug for Block {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{{...}}");
    }
}



macro c {
    ($colour:ident, $expr:expr) => {
        format!("{}{}\x1b[39m", $colour, $expr)
    }
}
