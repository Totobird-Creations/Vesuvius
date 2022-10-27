use std::{
    fs,
    fmt
};

use peg;


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
        return write!(f, "{}", self.declarations.iter().map(|x| x.fmt(0)).collect::<Vec<String>>().join("\n"));
    }
}


pub struct Declaration {
    headers     : Vec<HeaderType>,
    declaration : DeclarationType
}
impl Declaration {
    pub fn from(headers : Vec<HeaderType>, declaration : DeclarationType) -> Declaration {
        return Declaration {
            headers,
            declaration
        };
    }
}
impl Declaration {
    fn fmt(&self, indent : usize) -> String {
        return format!("{}{};", "  ".repeat(indent), self.declaration.fmt(indent));
    }
}


#[derive(Clone)]
pub enum DeclarationType {
    Import(
        DeclarationImportPart
    ),
    InitVar(
        bool,      // Public
        bool,      // Mutable
        String,    // Name
        Expression // Value
    )
}
impl DeclarationType {
    fn fmt(&self, indent : usize) -> String {
        return match (self) {
            DeclarationType::Import  (main)                         => format!("use {}", main.fmt(indent)),
            DeclarationType::InitVar (public, mutable, name, value) => format!("{} {} ={}> {}", if (*public) {"pub"} else {""}, name, if (*mutable) {"+"} else {""}, value.fmt(indent))
        };
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
            DeclarationImportPart::List (subs)       => format!("{{\n{}\n{}}}", subs.iter().map(|x| format!("{}{}", "  ".repeat(indent + 1), x.fmt(indent + 1))).collect::<Vec<String>>().join(",\n"), "  ".repeat(indent)),
            DeclarationImportPart::All               => String::from("*")
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
            DeclarationImportPartMode::Rename (to)  => format!("={}", to),
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
    Expression(Expression)
}
impl Statement {
    fn fmt(&self, indent : usize) -> String {
        return format!("{};", match (self) {
            Statement::Declaration (declaration) => declaration.fmt(indent),
            Statement::Expression  (expression)  => expression.fmt(indent)
        });
    }
}


#[derive(Clone)]
pub enum Expression {

    Function(
        Vec<(String, Type)>, // Arguments
        Option<Type>,        // Return
        Vec<Statement>       // Body
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
        Box<Expression>,
        String
    ),
    DotAccess(
        Box<Expression>,
        String
    ),
    Call(
        Box<Expression>,     // Base
        Box<Vec<Expression>> // Arguments
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
                args.iter().map(|(name, typ)| format!("{} : {}", name, typ.fmt(indent))).collect::<Vec<String>>().join(", "),
                if let Some(ret) = ret {format!(" {}", ret.fmt(indent))} else {String::new()},
                body.iter().map(|x| format!("{}{}", "  ".repeat(indent + 1), x.fmt(indent + 1))).collect::<Vec<String>>().join("\n"),
                "  ".repeat(indent)
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
    )
}
impl ExpressionModifier {
    pub fn to_expression(self, of : Expression) -> Expression {
        return match (self) {
            ExpressionModifier::StaticAccess (name) => Expression::StaticAccess(Box::new(of), name),
            ExpressionModifier::DotAccess    (name) => Expression::DotAccess(Box::new(of), name),
            ExpressionModifier::Call         (args) => Expression::Call(Box::new(of), Box::new(args))
        };
    }
}


#[derive(Clone)]
pub struct Type {
    parts : Vec<String>
}
impl Type {
    fn fmt(&self,_indent : usize) -> String {
        return self.parts.join("::");
    }
}



pub fn read(path : &str) -> Program {
    match (parser::program(&fs::read_to_string(path).unwrap())) {

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

            println!("\n{:?}\n{}\n",
                error.location,
                expected.join("\n")
            );
            std::process::exit(1);

        }

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
            = _ d:(declaration_import() / declaration_initvar()) _
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
            / _ i:ident() _ "=" _ r:ident() _
                {DeclarationImportPart::Name(i, DeclarationImportPartMode::Rename(r))}
            / _ i:ident() _
                {DeclarationImportPart::Name(i, DeclarationImportPartMode::None)}
            / _ "{" _ d:((_ d:declaration_import_part() _ {d}) ** ",") _ "}" _
                {DeclarationImportPart::List(Box::new(d))}

                                         
        rule declaration_initvar() -> DeclarationType
            = _ p:"pub"? _ n:ident() _ "=" m:"+"? ">" _ e:expression() _
                {DeclarationType::InitVar(matches!(p, Some(_)), matches!(m, Some(_)), n, e)}


        rule statement() -> Statement
            = _ d:declaration() _
                {Statement::Declaration(d)}
            / _ e:expression() _
                {Statement::Expression(e)}


        rule expression() -> Expression
            = _ "|" _ "|" _ r:typ()? _ "{" _ s:(_ s:statement() _ ";" _ {s})* _ "}" _
                {Expression::Function(Vec::new(), r, s)}
            / _ e:expression_addition() _
                {e}
            / _ "~" _ e:expression()
                {Expression::Return(Box::new(e))}

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

        rule expression_literal() -> Expression
            = _ i:int() _
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
            = i:((_ i:ident() _ {i}) ++ "::")
                {Type {
                    parts : i
                }}

        rule _()
            = quiet!{(" " / "\t" / "\n" / "\r")*}


    }
}
