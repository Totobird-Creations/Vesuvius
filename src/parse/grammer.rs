use std::path::PathBuf;

use peg::{
    parser,
    error::ParseError,
    str::LineCol
};

use crate::parse::node::*;


pub(crate) fn parse(text : String, fname : &PathBuf) -> Result<Program, ParseError<LineCol>> {
    return parser::parse(&text, fname);
}


parser! {grammar parser(fname : &PathBuf) for str {

    // Debug peg stuff
    pub(crate) rule parse() -> Program = traced(<program()>)
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

    rule program() -> Program
        = _ decls:(decl:declaration() _ ";" _ {decl})* ![_]
            {Program {
                decls
            }}


    rule declaration() -> Declaration
        = headers:(header:declaration_header() _ {header})* _
          vis:declaration_visibility()
          start:position!() decl:(declaration_import() / declaration_function()) end:position!() _
            {Declaration {
                headers,
                vis,
                decl,
                range   : Range(fname.clone(), start, end)
            }}

    rule declaration_header() -> DeclarationHeader
        = start:position!() "#[" header:(
            "entry" {DeclarationHeaderType::Entry}
        )"]" end:position!()
            {DeclarationHeader {
                header,
                range  : Range(fname.clone(), start, end)
            }}

    rule declaration_visibility() -> DeclarationVisibility
        = start:position!() vis:(
              ("pub" __)  {DeclarationVisibilityType::Public}
            / ("priv" __) {DeclarationVisibilityType::Private}
        )? end:position!()
            {
                let vis = if let Some(vis) = vis {vis} else {DeclarationVisibilityType::Private};
                DeclarationVisibility {
                    vis,
                    range : Range(fname.clone(), start, end)
                }
            }


    rule declaration_import() -> DeclarationType
        = "mod" __ start:position!() parts:((part:ident() _ {part}) ++ "::") end:position!()
            {DeclarationType::Module(parts, Range(fname.clone(), start, end))}

    rule declaration_function() -> DeclarationType
        = "fn" __ start:position!() name:ident() end:position!() _ /* TODO : Arguments and return */ block:block()
            {DeclarationType::Function(name, Range(fname.clone(), start, end), Vec::new(), None, block)}



    rule statement() -> Statement
        = start:position!() stmt:("let" __ start_name:position!() name:ident() end_name:position!() _ "=" _ value:expression()
            {StatementType::InitVar(name, Range(fname.clone(), start_name, end_name), value)}
        / expr:expression()
            {StatementType::Expression(expr)}
        ) end:position!() {Statement {
            stmt,
            range : Range(fname.clone(), start, end)
        }}



    rule expression() -> Expression
        = expr:expression_compare()
            {expr}


    rule expression_compare() -> Expression
    = left:expression_addition() _ ops:(op:$("==" / "!=" / ">" / ">=" / "<" / "<=") _ right:expression_addition() _ {(op, right)})*
        {
            let mut left = left;
            for (op, right) in ops {
                let range = Range(fname.clone(), left.range.1, right.range.2);
                left = Expression {
                    expr : match (op) {
                        "==" => ExpressionType::EqualsOperation(Box::new(left), Box::new(right)),
                        "!=" => ExpressionType::NotEqualsOperation(Box::new(left), Box::new(right)),
                        ">"  => ExpressionType::GreaterOperation(Box::new(left), Box::new(right)),
                        ">=" => ExpressionType::GreaterEqualsOperation(Box::new(left), Box::new(right)),
                        "<"  => ExpressionType::LessOperation(Box::new(left), Box::new(right)),
                        "<=" => ExpressionType::LessEqualsOperation(Box::new(left), Box::new(right)),
                        _    => panic!("INTERNAL ERROR")
                    },
                    range
                }
            }
            left
        }

    rule expression_addition() -> Expression
    = left:expression_multiply() _ ops:(op:$("+" / "-") _ right:expression_multiply() _ {(op, right)})*
        {
            let mut left = left;
            for (op, right) in ops {
                let range = Range(fname.clone(), left.range.1, right.range.2);
                left = Expression {
                    expr : match (op) {
                        "+" => ExpressionType::AdditionOperation(Box::new(left), Box::new(right)),
                        "-" => ExpressionType::SubtractionOperation(Box::new(left), Box::new(right)),
                        _   => panic!("INTERNAL ERROR")
                    },
                    range
                }
            }
            left
        }

    rule expression_multiply() -> Expression
        = left:atom() _ ops:(op:$("*" / "/") _ right:atom() _ {(op, right)})*
            {
                let     left_range = left.range.clone();
                let mut left       = Expression {
                    expr  : ExpressionType::Atom(left),
                    range : left_range
                };
                for (op, right) in ops {
                    let right_range = right.range.clone();
                    let right       = Expression {
                        expr  : ExpressionType::Atom(right),
                        range : right_range
                    };
                    let range = Range(fname.clone(), left.range.1, right.range.2);
                    left = Expression {
                        expr : match (op) {
                            "*" => ExpressionType::MultiplicationOperation(Box::new(left), Box::new(right)),
                            "/" => ExpressionType::DivisionOperation(Box::new(left), Box::new(right)),
                            _   => panic!("INTERNAL PANIC")
                        },
                        range
                    }
                }
                left
            }




    rule atom() -> Atom
        = start:position!() atom:("(" _ expr:expression() _ ")"
            {AtomType::Expression(Box::new(expr))}
        / atom:atom_if()
            {atom}
        / lit:literal()
            {AtomType::Literal(lit)}
        ) end:position!() {Atom {
            atom  : atom,
            range : Range(fname.clone(), start, end)
        }}
    
    rule atom_if() -> AtomType
        = ifstart:position!() "if" _ "(" _ ifcondi:expression() _ ")" _ ifblock:block() ifend:position!() _
          elf:(elifstart:position!() "elif" _ "(" _ elifcondi:expression() _ ")" _ elifblock:block() elifend:position!() _ {(Box::new(elifcondi), elifblock, Range(fname.clone(), elifstart, elifend))})*
          els:(elsestart:position!() "else" _ elseblock:block() elseend:position!() {(elseblock, Range(fname.clone(), elsestart, elseend))})?
            {
                let mut ifs  = vec![(Box::new(ifcondi), ifblock, Range(fname.clone(), ifstart, ifend))];
                let mut elf = elf;
                ifs.append(&mut elf);
                AtomType::If(
                    ifs,
                    els
                )
            }


    rule literal() -> Literal
        = start:position!() lit:(ident:ident()
            {LiteralType::Identifier(ident)}
        / int:['0'..='9']+ dec:("." b:['0'..='9']+ {b})?
            {if let Some(dec) = dec {
                LiteralType::Float(int.into_iter().collect(), dec.into_iter().collect())
            } else {
                LiteralType::Int(int.into_iter().collect())
            }}
        ) end:position!() {Literal {
            lit,
            range : Range(fname.clone(), start, end)
        }}

    rule block() -> Block
        = start:position!() "{" _ b:(s:((_ s:statement() _ {s}) ++ ";") r:";"? {(s, r)})? _ "}" end:position!()
            {
                let range = Range(fname.clone(), start, end);
                if let Some(body) = b {
                    Block {
                        stmts   : body.0,
                        retlast : matches!(body.1, None),
                        range
                    }
                } else {
                    Block {
                        stmts   : Vec::new(),
                        retlast : false,
                        range
                    }
                }
            }

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
