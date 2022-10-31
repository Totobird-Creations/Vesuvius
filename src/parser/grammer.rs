use peg::{
    parser,
    error::ParseError,
    str::LineCol
};

use crate::parser::node::*;


pub fn parse(text : String) -> Result<Program, ParseError<LineCol>> {
    return parser::traced_parse(&text);
}


parser! {pub grammar parser() for str {

    pub rule traced_parse() -> Program = traced(<parse()>)
    rule traced<T>(e: rule<T>) -> T =
        &(input:$([_]*) {
            #[cfg(feature = "trace")]
            println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
        })
        e:e()? {?
            #[cfg(feature = "trace")]
            println!("[PEG_TRACE_STOP]");
            e.ok_or("")
        }

    pub rule parse() -> Program
        = _ decls:(decl:declaration() _ ";" _ {decl})* ![_]
            {Program {
                decls
            }}


    rule declaration() -> Declaration
        = headers:(header:declaration_header() _ {header})* _
          vis:declaration_visibility()
          decl:(declaration_function()) _
            {Declaration {
                headers,
                vis,
                decl
            }}
    rule declaration_header() -> DeclarationHeader
        = "#[" header:(
            "entry" {DeclarationHeader::Entry}
        )"]"
            {header}
    rule declaration_visibility() -> DeclarationVisibility
        = visibility:(
              ("pub" __)  {DeclarationVisibility::Public}
            / ("priv" __) {DeclarationVisibility::Private}
        )?
            {if let Some(visibility) = visibility {visibility} else {DeclarationVisibility::Private}}


    rule declaration_function() -> DeclarationType
        = "fn" __ name:ident() _ /* TODO : Arguments and return */ block:block()
            {DeclarationType::Function(name, Vec::new(), None, block)}



    rule statement() -> Statement
        = "let" __ name:ident() _ "=" _ value:expression()
            {Statement::InitVar(name, value)}
        / expr:expression()
            {Statement::Expression(expr)}



    rule expression() -> Expression
        = expr:expression_compare()
            {expr}


    rule expression_compare() -> Expression
    = left:expression_addition() _ ops:(op:$("==" / "!=" / ">" / ">=" / "<" / "<=") _ right:expression_addition() _ {(op, right)})*
        {
            let mut left = left;
            for (op, right) in ops {
                match (op) {
                    "==" => left = Expression::EqualsOperation(Box::new(left), Box::new(right)),
                    "!=" => left = Expression::NotEqualsOperation(Box::new(left), Box::new(right)),
                    ">"  => left = Expression::GreaterOperation(Box::new(left), Box::new(right)),
                    ">=" => left = Expression::GreaterEqualsOperation(Box::new(left), Box::new(right)),
                    "<"  => left = Expression::LessOperation(Box::new(left), Box::new(right)),
                    "<=" => left = Expression::LessEqualsOperation(Box::new(left), Box::new(right)),
                    _    => panic!("INTERNAL ERROR")
                }
            }
            left
        }

    rule expression_addition() -> Expression
    = left:expression_multiply() _ ops:(op:$("+" / "-") _ right:expression_multiply() _ {(op, right)})*
        {
            let mut left = left;
            for (op, right) in ops {
                match (op) {
                    "+" => left = Expression::AdditionOperation(Box::new(left), Box::new(right)),
                    "-" => left = Expression::SubtractionOperation(Box::new(left), Box::new(right)),
                    _   => panic!("INTERNAL ERROR")
                }
            }
            left
        }

    rule expression_multiply() -> Expression
        = left:atom() _ ops:(op:$("*" / "/") _ right:atom() _ {(op, right)})*
            {
                let mut left = Expression::Atom(left);
                for (op, right) in ops {
                    let right = Expression::Atom(right);
                    match (op) {
                        "*" => left = Expression::MultiplicationOperation(Box::new(left), Box::new(right)),
                        "/" => left = Expression::DivisionOperation(Box::new(left), Box::new(right)),
                        _   => panic!("INTERNAL PANIC")
                    }
                }
                left
            }




    rule atom() -> Atom
        = "(" _ expr:expression() _ ")"
            {Atom::Expression(Box::new(expr))}
        / atom:atom_if()
            {atom}
        / lit:literal()
            {Atom::Literal(lit)}
    
    rule atom_if() -> Atom
        = "if" _ "(" _ ifcondi:expression() _ ")" _ ifblock:block() _
          elf:("elif" _ "(" _ elifcondi:expression() _ ")" _ elifblock:block() _ {(Box::new(elifcondi), elifblock)})*
          els:("else" _ elseblock:block() {elseblock})?
            {
                let mut ifs  = vec![(Box::new(ifcondi), ifblock)];
                let mut elf = elf;
                ifs.append(&mut elf);
                Atom::If(
                    ifs,
                    els
                )
            }


    rule literal() -> Literal
        = ident:ident()
            {Literal::Identifier(ident)}
        / int:['0'..='9']+ dec:("." b:['0'..='9']+ {b})?
            {if let Some(dec) = dec {
                Literal::Float(int.into_iter().collect(), dec.into_iter().collect())
            } else {
                Literal::Int(int.into_iter().collect())
            }}

    rule block() -> Block
        = "{" _ b:(s:((_ s:statement() _ {s}) ++ ";") r:";"? {(s, r)})? _ "}"
            {if let Some(body) = b {
                Block {
                    stmts   : body.0,
                    retlast : matches!(body.1, None)
                }
            } else {
                Block {
                    stmts   : Vec::new(),
                    retlast : false
                }
            }}

    rule ident() -> String
        = i:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {String::from(i)}


    rule _()
        = ___*
    rule __()
        = ___+
    rule ___()
        = quiet! {
              [' ' | '\t' | '\r' | '\n']
            / "//" single_comment()
            / "/*" multi_comment()
        }
    rule single_comment()
        = (
              "/*" multi_comment()
            / [^ '\n']
        )* ("\n" / ![_])
    rule multi_comment()
        = (
              "//" single_comment()
            / "/*" multi_comment()
            / [^ '*']
        )* "*" (
              "/"
            / [^ '/'] multi_comment()
        )

}}
