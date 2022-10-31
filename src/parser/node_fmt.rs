use std::fmt::{
    Display,
    Formatter,
    Result
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
        return write!(f, "{}", self.decls.iter().map(|decl| format!("{};", decl.fmt(0))).collect::<Vec<String>>().join("\n"));
    }
}


impl Declaration {
    fn fmt(&self, indent : usize) -> String {
        return format!("{}{} {}",
            self.headers.iter()
                .map(|header| format!("{}\n{}",
                    header.fmt(indent),
                    INDENT.repeat(indent)
                ))
                .collect::<Vec<String>>()
                .join(""),
            self.vis.fmt(indent),
            self.decl.fmt(indent)
        );
    }
}


impl DeclarationHeader {
    fn fmt(&self, _indent : usize) -> String {
        return c!(HEADER, format!("#[{}]", match (self) {
            Self::Entry => "entry"
        }));
    }
}


impl DeclarationVisibility {
    fn fmt(&self, _indent : usize) -> String {
        return c!(KEYWORD, match (self) {
            Self::Public  => "pub",
            Self::Private => "priv"
        });
    }
}


impl DeclarationType {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {

            Self::Function(name, _args, _ret, block) => {
                format!("{} {} {}",
                    c!(OBJECT, "fn"),
                    c!(NAME, name),
                    block.fmt(indent)
                )
            }

        };
    }
}


impl Statement {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {

            Self::InitVar(name, value) => {
                format!("{} {} = {}",
                    c!(KEYWORD, "let"),
                    c!(NAME, name),
                    value.fmt(indent)
                )
            },

            Self::Expression(expr) => expr.fmt(indent)

        }
    }
}


impl Expression {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {

            Self::EqualsOperation         (left, right) => format!("({} == {})", left.fmt(indent), right.fmt(indent)),
            Self::NotEqualsOperation      (left, right) => format!("({} != {})", left.fmt(indent), right.fmt(indent)),
            Self::GreaterOperation        (left, right) => format!("({} > {})", left.fmt(indent), right.fmt(indent)),
            Self::GreaterEqualsOperation  (left, right) => format!("({} >= {})", left.fmt(indent), right.fmt(indent)),
            Self::LessOperation           (left, right) => format!("({} < {})", left.fmt(indent), right.fmt(indent)),
            Self::LessEqualsOperation     (left, right) => format!("({} <= {})", left.fmt(indent), right.fmt(indent)),
            Self::AdditionOperation       (left, right) => format!("({} + {})", left.fmt(indent), right.fmt(indent)),
            Self::SubtractionOperation    (left, right) => format!("({} - {})", left.fmt(indent), right.fmt(indent)),
            Self::MultiplicationOperation (left, right) => format!("({} * {})", left.fmt(indent), right.fmt(indent)),
            Self::DivisionOperation       (left, right) => format!("({} / {})", left.fmt(indent), right.fmt(indent)),

            Self::Atom(atom) => atom.fmt(indent)

        }
    }
}


impl Atom {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {

            Self::Literal(lit) => lit.fmt(indent),

            Self::Expression(expr) => expr.fmt(indent),

            Self::If(ifs, els) => {
                format!("{}{}{}",
                    c!(KEYWORD, "if"),
                    ifs.iter()
                        .map(|i| format!(" ({}) {}",
                            i.0.fmt(indent),
                            i.1.fmt(indent)
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
                            els.fmt(indent)
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
    fn fmt(&self, _indent : usize) -> String {
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
    fn fmt(&self, indent : usize) -> String {
        return format!("{{{}{}\n{}}}",
            self.stmts.iter()
                .map(|stmt| format!("\n{}{}",
                    INDENT.repeat(indent + 1),
                    stmt.fmt(indent + 1)
                ))
                .collect::<Vec<String>>()
                .join(";"),
            if (self.retlast) {""} else {";"},
            INDENT.repeat(indent)
        );
    }
}



macro c {
    ($colour:ident, $expr:expr) => {
        format!("{}{}\x1b[39m", $colour, $expr)
    }
}
