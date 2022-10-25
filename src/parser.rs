use std::fs;

use peg;


pub struct Declaration {
    headers     : Vec<HeaderType>,
    declaration : DeclarationType
}
impl Declaration {
    pub fn from(headers : Vec<HeaderType>, declaration : DeclarationType) -> Declaration {
        return Declaration {
            headers     : headers,
            declaration : declaration
        };
    }
}
pub enum DeclarationType {
    Import(
        String // Main module.
    ),
    InitVar(
        bool,      // Mutable
        String,    // Name
        Expression // Value
    )
}
pub enum HeaderType {
    Entry,
    Public
}

pub enum Expression {}


pub fn read(path : &str) -> Vec<Declaration> {
    return parser::program(&fs::read_to_string(path).unwrap()).unwrap();
}


peg::parser! {
    grammar parser() for str {

        pub rule program() -> Vec<Declaration>
            = e:(declaration() ** ";")
            {e}

        rule declaration() -> Declaration
            = _ h:declaration_headers() _ d:(declaration_import() / declaration_initvar()) _
            {Declaration::from(h, d)}

        rule declaration_headers() -> Vec<HeaderType>
            = _ h:(declaration_header())* _
            {h.into_iter().flatten().collect::<Vec<HeaderType>>()}
        rule declaration_header() -> Vec<HeaderType>
            = _ "#[" _ e:(declaration_header_entry() ++ ",") _ "]" _
            {e}
        rule declaration_header_entry() -> HeaderType
            = _
              "entry" {HeaderType::Entry}
            / "pub"   {HeaderType::Public}

        rule declaration_import() -> DeclarationType
            = _ "import" _ e:ident() _
            {DeclarationType::Import(e)}
        rule declaration_initvar() -> DeclarationType
            = _ m:$("cst" / "var") _ n:ident() _ e:expression() _
            {DeclarationType::InitVar(m == String::from("var"), n, e)}

        rule expression() -> Expression
            = "|"
            {/* TODO */}


        rule ident() -> String
            = i:['a'..='z' | 'A'..='Z' | '0'..='9']*
            {i.iter().collect::<String>()}

        rule _()
            = quiet!{(" " / "\t" / "\n" / "\r")*}

    }
}
