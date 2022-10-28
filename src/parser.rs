use std::{
    fs,
    fmt
};

use peg;



pub fn read(path : &str) -> Program {
    let file = match (fs::read_to_string(path)) {
        Ok  (text)  => text,
        Err (error) => {
            println!("\x1b[31m\x1b[1mFAILED TO READ SCRIPT FILE!\x1b[0m");
            println!("  \x1b[31m{}\x1b[0m", error);
            std::process::exit(1);
        }
    };

    match (parser::program(&file)) {

        Ok(program) => {
            
            println!("\n{}\n",  program);
            return program;

        },

        Err(error) => {

            let mut expected = error.expected
                .tokens()
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
            expected.sort();

            println!("\x1b[31m\x1b[1mFAILED TO PARSE SCRIPT\x1b[0m");
            println!("  \x1b[33mFailed at lin \x1b[1m{}\x1b[0m\x1b[33m, col \x1b[1m{}\x1b[0m\x1b[33m, off \x1b[1m{}\x1b[0m\x1b[33m \x1b[0m", error.location.line, error.location.column, error.location.offset);

            /*println!("\n{:?}\n{}\n",
                error.location,
                expected.join("\n")
            );*/
            std::process::exit(1);

        }

    }
}



pub struct Program {
    declarations : Vec<Declaration>
}
impl Program {
    pub fn from(declarations : Vec<Declaration>) -> Program {
        return Program {
            declarations
        };
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f,
            "{}",
            self.declarations.iter()
                .map(|x| x.fmt(0))
                .collect::<Vec<String>>().join("\n")
                .split("\n")
                .filter(|x| x.replace(" ", "").len() > 0)
                .collect::<Vec<&str>>()
                .join("\n")
        );
    }
}


pub struct Declaration {
    _headers     : Vec<HeaderType>,
    declaration  : DeclarationType
}
impl Declaration {
    pub fn from(headers : Vec<HeaderType>, declaration : DeclarationType) -> Declaration {
        return Declaration {
            _headers : headers,
            declaration
        };
    }
}
impl Declaration {
    fn fmt(&self, indent : usize) -> String {
        return format!(
            "{}{};",
            "  ".repeat(indent),
            self.declaration.fmt(indent)
        );
    }
}


#[derive(Clone)]
pub enum DeclarationType {
    Import(
        DeclarationImportPart
    ),
    InitVar(DeclarationInitVar),
}
impl DeclarationType {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {
            DeclarationType::Import  (main)    => format!("use {}", main.fmt(indent)),
            DeclarationType::InitVar (initvar) => initvar.fmt(indent)
        };
    }
}

#[derive(Clone)]
pub struct DeclarationInitVar {
    public  : bool,
    mutable : bool,
    name    : String,
    value   : Expression
}
impl DeclarationInitVar {
    fn fmt(&self, indent : usize) -> String {
        return format!("{}let{} {} = {}",
            if (self.public) {"pub "} else {""},
            if (self.mutable) {" mut"} else {""},
            self.name,
            self.value.fmt(indent)
        );
    }
}

#[derive(Clone)]
pub enum DeclarationImportPart {
    Name(
        String,                   // Source Name
        DeclarationImportPartMode
    ),
    List(Box<Vec<DeclarationImportPart>>),
    All
}
impl DeclarationImportPart {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {
            DeclarationImportPart::Name (name, mode) => format!("{}{}", name, mode.fmt(indent)),
            DeclarationImportPart::List (subs)       => format!(
                "{{\n{}\n{}}}",
                subs.iter()
                    .map(|x| format!(
                        "{}{}",
                        "  ".repeat(indent + 1),
                        x.fmt(indent + 1)
                    )).collect::<Vec<String>>().join(",\n"),
                "  ".repeat(indent)
            ),
            DeclarationImportPart::All => String::from("*")
        };
    }
}
#[derive(Clone)]
pub enum DeclarationImportPartMode {
    None,
    Rename(String),
    Sub(Box<DeclarationImportPart>)
}
impl DeclarationImportPartMode {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {
            DeclarationImportPartMode::None         => String::new(),
            DeclarationImportPartMode::Rename (to)  => format!(" = {}", to),
            DeclarationImportPartMode::Sub    (sub) => format!("::{}", sub.fmt(indent))
        };
    }
}


pub enum HeaderType {
    Entry,
    Public
}


#[derive(Clone)]
pub enum Statement {
    Declaration(DeclarationType),
    SetVar(Expression, Expression),
    Expression(Expression)
}
impl Statement {
    fn fmt(&self, indent : usize) -> String {
        return format!("{};", match (self) {
            Statement::Declaration (declaration) => declaration.fmt(indent),
            Statement::Expression  (expression)  => expression.fmt(indent),
            Statement::SetVar      (name, value) => format!("{} = {}", name.fmt(indent), value.fmt(indent))
        });
    }
}


#[derive(Clone)]
pub enum Expression {

    Function(
        Vec<(String, Type)>,   // Arguments
        Option<Type>,          // Return
        Option<Vec<Statement>> // Body
    ),
    Struct(
        Vec<(            // Generics
            String,
            Option<Type>
        )>,
        Option<Type>,    // Extends,
        Vec<(            // Values
            bool,          // Public
            String,        // Name
            Type           // Type
        )>
    ),
    Trait(
        Vec<(                   // Generics
            String,
            Option<Type>
        )>,
        Option<Type>,           // Extends,
        Vec<DeclarationInitVar> // Functions
    ),
    TraitImpl(
        Type,                   // Struct
        Type,                   // Trait
        Vec<DeclarationInitVar> // Functions
    ),

    Inject(
        Box<Expression>,
        Type
    ),

    Equals(
        Box<Expression>,
        Box<Expression>
    ),
    NotEquals(
        Box<Expression>,
        Box<Expression>
    ),
    Greater(
        Box<Expression>,
        Box<Expression>
    ),
    GreaterEquals(
        Box<Expression>,
        Box<Expression>
    ),
    Less(
        Box<Expression>,
        Box<Expression>
    ),
    LessEquals(
        Box<Expression>,
        Box<Expression>
    ),

    Addition(
        Box<Expression>,
        Box<Expression>
    ),
    Subtraction(
        Box<Expression>,
        Box<Expression>
    ),
    Multiplication(
        Box<Expression>,
        Box<Expression>
    ),
    Division(
        Box<Expression>,
        Box<Expression>
    ),

    StaticAccess(
        Box<Expression>, // Base
        String           // Subname
    ),
    DotAccess(
        Box<Expression>, // Base
        String           // Subname
    ),
    Call(
        Box<Expression>,     // Base
        Box<Vec<Expression>> // Arguments
    ),
    Build(
        Box<Expression>, // Base
        Box<Vec<(
            String,      // Argument Name
            Expression   // Argument Value
        )>>
    ),
    Generics(
        Box<Expression>, // Base
        Vec<Type>        // Types
    ),

    Integer(i64),
    VarAccess(String),
    String(String),

    Return(Box<Expression>)

}
impl Expression {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {

            Expression::Function(args, ret, body) => format!(
                "(|{}|{} {{\n{}\n{}}})",
                args.iter().map(
                    |(name, typ)| format!("{} : {}", name, typ.fmt(indent))
                ).collect::<Vec<String>>().join(", "),
                if let Some(ret) = ret {
                    format!(" {}", ret.fmt(indent))
                } else {String::new()},
                if let Some(body) = body {
                    body.iter()
                        .map(|x| format!("{}{}", "  ".repeat(indent + 1), x.fmt(indent + 1)))
                        .collect::<Vec<String>>().join("\n")
                } else {String::new()},
                "  ".repeat(indent)
            ),
            Expression::Struct(generics, extends, args) => format!(
                "struct{}{} {{\n{}\n{}}}",
                if (generics.len() > 0) {
                    format!("<{}>", generics.iter()
                        .map(|(name, typ)| format!(
                            "{}{}",
                            name,
                            if let Some(typ) = typ {
                                format!(" : {}", typ.fmt(indent))
                            } else {String::new()}
                        )).collect::<Vec<String>>().join(", ")
                    )
                } else {String::new()},
                if let Some(extends) = extends {
                    format!(" : {}", extends.fmt(indent))
                } else {String::new()},
                args.iter()
                    .map(|(public, name, typ)| format!(
                        "{}{}{} : {}",
                        "  ".repeat(indent + 1),
                        if (*public) {"pub "} else {""},
                        name,
                        typ.fmt(indent + 1)
                    )).collect::<Vec<String>>().join("\n"),
                "  ".repeat(indent)
            ),
            Expression::Trait(generics, extends, functions) => format!(
                "trait{}{} {{\n{}\n{}}}",
                if (generics.len() > 0) {
                    format!("<{}>", generics.iter()
                        .map(|(name, typ)| format!(
                            "{}{}", 
                            name,
                            if let Some(typ) = typ {
                                format!(" : {}", typ.fmt(indent))
                            } else {String::new()}
                        )).collect::<Vec<String>>().join(", ")
                    )
                } else {String::new()},
                if let Some(extends) = extends {
                    format!(" : {}", extends.fmt(indent))
                } else {String::new()},
                functions.iter()
                    .map(|function| format!(
                        "{}{};",
                        "  ".repeat(indent + 1),
                        function.fmt(indent + 1)
                    ))
                    .collect::<Vec<String>>().join("\n"),
                "  ".repeat(indent)
            ),
            Expression::TraitImpl(struc, trai, functions) => format!(
                "impl {} : {} {{\n{}\n{}}}" ,
                struc.fmt(indent),
                trai.fmt(indent),
                functions.iter()
                    .map(|function| format!(
                        "{}{};",
                        "  ".repeat(indent + 1),
                        function.fmt(indent + 1)
                    ))
                    .collect::<Vec<String>>().join("\n"),
                "  ".repeat(indent)
            ),

            Expression::Inject(object, typ) => format!(
                "({} << {})",
                object.fmt(indent),
                typ.fmt(indent)
            ),

            Expression::Equals(left, right) => format!(
                "({} == {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::NotEquals(left, right) => format!(
                "({} != {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::Greater(left, right) => format!(
                "({} > {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::GreaterEquals(left, right) => format!(
                "({} >= {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::Less(left, right) => format!(
                "({} < {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::LessEquals(left, right) => format!(
                "({} <= {})",
                left.fmt(indent),
                right.fmt(indent)
            ),

            Expression::Addition(left, right) => format!(
                "({} + {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::Subtraction(left, right) => format!(
                "({} - {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::Multiplication(left, right) => format!(
                "({} * {})",
                left.fmt(indent),
                right.fmt(indent)
            ),
            Expression::Division(left, right) => format!(
                "({} / {})",
                left.fmt(indent),
                right.fmt(indent)
            ),

            Expression::StaticAccess(of, name) => format!(
                "{}::{}",
                of.fmt(indent),
                name
            ),
            Expression::DotAccess(of, name) => format!(
                "{}.{}",
                of.fmt(indent),
                name
            ),
            Expression::Call(of, args) => format!(
                "{}(\n{}\n{})",
                of.fmt(indent),
                args.iter().map(|x| format!("{}{}", "  ".repeat(indent + 1), x.fmt(indent + 1))).collect::<Vec<String>>().join(",\n"),
                "  ".repeat(indent)
            ),
            Expression::Build(of, values) => format!(
                "{} {{\n{}\n{}}}",
                of.fmt(indent),
                values.iter().map(|(name, value)| format!("{}{} : {}", "  ".repeat(indent + 1), name, value.fmt(indent + 1))).collect::<Vec<String>>().join(",\n"),
                "  ".repeat(indent)
            ),
            Expression::Generics(of, types) => format!(
                "{}<{}>",
                of.fmt(indent),
                types.iter().map(|typ| typ.fmt(indent)).collect::<Vec<String>>().join(",")
            ),

            Expression::Integer   (value) => value.to_string(),
            Expression::VarAccess (name)  => name.clone(),
            Expression::String    (value) => format!("\"{}\"", value),

            Expression::Return (value) => format!("~{}", value.fmt(indent))

        };
    }
}

pub enum ExpressionModifier {
    StaticAccess(
        String
    ),
    DotAccess(
        String
    ),
    Call(
        Vec<Expression> // Arguments
    ),
    Build(
        Vec<(
            String,    // Name
            Expression // Value
        )>
    ),
    Generics(
        Vec<Type>
    )
}
impl ExpressionModifier {
    pub fn to_expression(self, of : Expression) -> Expression {
        return match (self) {
            ExpressionModifier::StaticAccess (name)   => Expression::StaticAccess(Box::new(of), name),
            ExpressionModifier::DotAccess    (name)   => Expression::DotAccess(Box::new(of), name),
            ExpressionModifier::Call         (args)   => Expression::Call(Box::new(of), Box::new(args)),
            ExpressionModifier::Build        (values) => Expression::Build(Box::new(of), Box::new(values)),
            ExpressionModifier::Generics     (types)  => Expression::Generics(Box::new(of), types)
        };
    }
}


#[derive(Clone)]
pub struct Type {
    refer    : bool,
    parts    : Vec<String>,
    generics : Box<Vec<Type>>
}
impl Type {
    fn fmt(&self, indent : usize) -> String {
        return format!(
            "{}{}{}",
            if (self.refer) {"&"} else {""}, 
            self.parts.join("::"),
            if (self.generics.len() > 0) {format!("<{}>", self.generics.iter().map(|generic| generic.fmt(indent)).collect::<Vec<String>>().join(", "))} else {String::new()}
        );
    }
}


peg::parser! {
    grammar parser() for str {


        pub rule program() -> Program
            = _ e:(_ e:declaration_with_headers() _ ";" _ {e})* _
                {Program::from(e)}


        rule declaration_with_headers() -> Declaration
            = _ h:declaration_headers() _ d:declaration() _
                {Declaration::from(h, d)}

        rule declaration_headers() -> Vec<HeaderType>
            = _ h:(
                _ "#[" _ h:(
                    (_ h:(
                          "entry" {HeaderType::Entry}
                        / "pub"   {HeaderType::Public}
                    ) _ {h}) ++ ","
                ) _ "]" _ {h}
            )* _ {h.into_iter().flatten().collect::<Vec<HeaderType>>()}


        rule declaration() -> DeclarationType
            = _ d:(declaration_import() / d:declaration_initvar() {DeclarationType::InitVar(d)}) _
                {d}

        
        rule declaration_import() -> DeclarationType
            = _ "use" _ i:ident() _ d:("::" d:declaration_import_part() {d})? _
                {
                    if let Some(d) = d {
                        DeclarationType::Import(DeclarationImportPart::Name(i, DeclarationImportPartMode::Sub(Box::new(d))))
                    } else {
                        DeclarationType::Import(DeclarationImportPart::Name(i, DeclarationImportPartMode::None))
                    }
                }
        
        rule declaration_import_part() -> DeclarationImportPart
            = _ "*" _
                {DeclarationImportPart::All}
            / _ i:ident() _ "::" _ n:declaration_import_part() _
                {DeclarationImportPart::Name(i, DeclarationImportPartMode::Sub(Box::new(n)))}
            / _ r:ident() _ "=" _ i:ident() _
                {DeclarationImportPart::Name(i, DeclarationImportPartMode::Rename(r))}
            / _ i:ident() _
                {DeclarationImportPart::Name(i, DeclarationImportPartMode::None)}
            / _ "{" _ d:((_ d:declaration_import_part() _ {d}) ** ",") _ "}" _
                {DeclarationImportPart::List(Box::new(d))}

                                         
        rule declaration_initvar() -> DeclarationInitVar
            = _ p:"pub"? _ "let" _ m:"mut"? _ n:ident() _ "=" _ e:expression() _
                {DeclarationInitVar {
                    public  : matches!(p, Some(_)),
                    mutable : matches!(m, Some(_)),
                    name    : n,
                    value   : e
                }}


        rule statement() -> Statement
            = _ d:declaration() _
                {Statement::Declaration(d)}
            / _ n:expression() _ "=" v:expression() _
                {Statement::SetVar(n, v)}
            / _ e:expression() _
                {Statement::Expression(e)}


        rule expression() -> Expression
            = _ "|" _ a:((_ n:ident() _ ":" _ t:typ() _ {(n, t)}) ** ",") _ "|" _ r:typ()? _ b:(_ "{" _ s:(_ s:statement() _ ";" _ {s})* _ "}" _ {s})?
                {Expression::Function(a, r, b)}
            / _ "struct" _ g:("<" _ g:((_ n:ident() _ t:(":" _ t:typ() {t})? _ {(n, t)}) ** ",") _ ">" {g})? _ p:(_ ":" _ p:typ() _ {p})? _ "{" _ a:((_ p:"pub"? _ i:ident() _ ":" _ t:typ() _ {(matches!(p, Some(_)), i, t)}) ** ",") _ "}" _
                {Expression::Struct(
                    if let Some(generics) = g {generics} else {Vec::new()},
                    p,
                    a
                )}
            / _ "trait" _ g:("<" _ g:((_ n:ident() _ t:(":" _ t:typ() {t})? _ {(n, t)}) ** ",") _ ">" {g})? _ p:(_ ":" _ p:typ() _ {p})? _ "{" _ d:(_ d:declaration_initvar() _ ";" _ {d})* _ "}" _
                {Expression::Trait(
                    if let Some(generics) = g {generics} else {Vec::new()},
                    p,
                    d
                )}
            / _ "impl" _ s:typ() _ ":" _ t:typ() _ "{" d:(_ d:declaration_initvar() _ ";" _ {d})* "}" _
                {Expression::TraitImpl(s, t, d)}
            / _ e:expression_inject() _
                {e}
            / _ "~" _ e:expression() _
                {Expression::Return(Box::new(e))}

        rule expression_inject() -> Expression
            = _ o:expression_compare() _ "<<" _ t:typ() _
                {Expression::Inject(Box::new(o), t)}
            / _ e:expression_compare() _
                {e}

        rule expression_compare() -> Expression
            = _ a:expression_addition() _ "==" _ b:expression_compare() _
                {Expression::Equals(Box::new(a), Box::new(b))}
            / _ a:expression_addition() _ "!=" _ b:expression_compare() _
                {Expression::NotEquals(Box::new(a), Box::new(b))}
            / _ a:expression_addition() _ ">" _ b:expression_compare() _
                {Expression::Greater(Box::new(a), Box::new(b))}
            / _ a:expression_addition() _ ">=" _ b:expression_compare() _
                {Expression::GreaterEquals(Box::new(a), Box::new(b))}
            / _ a:expression_addition() _ "<" _ b:expression_compare() _
                {Expression::Less(Box::new(a), Box::new(b))}
            / _ a:expression_addition() _ "<=" _ b:expression_compare() _
                {Expression::LessEquals(Box::new(a), Box::new(b))}
            / _ e:expression_addition() _
                {e}

        rule expression_addition() -> Expression
            = _ a:expression_multiplication() _ "+" _ b:expression_addition() _
                {Expression::Addition(Box::new(a), Box::new(b))}
            / _ a:expression_multiplication() _ "-" _ b:expression_addition() _
                {Expression::Subtraction(Box::new(a), Box::new(b))}
            / _ e:expression_multiplication() _
                {e}

        rule expression_multiplication() -> Expression
            = _ a:expression_modifiable() _ "*" _ b:expression_multiplication() _
                {Expression::Multiplication(Box::new(a), Box::new(b))}
            / _ a:expression_modifiable() _ "/" _ b:expression_multiplication() _
                {Expression::Division(Box::new(a), Box::new(b))}
            / _ e:expression_modifiable() _
                {e}

        rule expression_modifiable() -> Expression
            = _ e:expression_literal() _ m:(m:expression_modifier())* _
                {
                    let mut expr = e;
                    for modifier in m {
                        expr = modifier.to_expression(expr);
                    }
                    expr
                }

        rule expression_modifier() -> ExpressionModifier
            = _ "::" _ i:ident() _
                {ExpressionModifier::StaticAccess(i)}
            / _ "." _ i:ident() _
                {ExpressionModifier::DotAccess(i)}
            / _ "(" _ a:((_ e:expression() _ {e}) ** ",") _ ")" _
                {ExpressionModifier::Call(a)}
            / _ "{" _ a:((_ n:ident() _ ":" _ v:expression() _ {(n, v)}) ** ",") _ "}" _
                {ExpressionModifier::Build(a)}
            / _ "<" _ g:((_ t:typ() _ {t}) ** ",") _ ">"
                {ExpressionModifier::Generics(g)}

        rule expression_literal() -> Expression
            = _ "(" _ e:expression() _ ")" _
                {e}
            / _ i:int() _
                {Expression::Integer(i)}
            / _ i:ident() _
                {Expression::VarAccess(i)}
            / _ "\"" s:(
                [^ '"' | '\\' | '\r' | '\n']
                / "\\n" {'\n'}
                / expected!("Valid escape sequence.")
            )* "\"" _
                {Expression::String(s.into_iter().collect())}


        rule ident() -> String
            = i:['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+
                {i.iter().collect::<String>()}

        rule int() -> i64
            = i:['0'..='9']+
                {i.iter().collect::<String>().parse::<i64>().unwrap()}

        rule typ() -> Type
            = r:"&"? _ i:((_ i:ident() _ {i}) ++ "::") _ g:(_ "<" _ g:((_ t:typ() _ {t}) ** ",") _ ">" _ {g})?
                {Type {
                    refer    : matches!(r, Some(_)),
                    parts    : i,
                    generics : Box::new(if let Some(generics) = g {generics} else {Vec::new()})
                }}

        rule _()
            = quiet!{(" " / "\t" / "\n" / "\r")*}


    }
}
