use std::{
    fs,
    fmt
};

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
impl fmt::Display for Declaration {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{};", self.declaration);
    }
}


#[derive(Clone)]
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
impl fmt::Display for DeclarationType {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", match (self) {
            DeclarationType::Import  (main)                 => format!("import {}", main),
            DeclarationType::InitVar (mutable, name, value) => format!("{} {} = {}", if (*mutable) {"var"} else {"cst"}, name, value)
        });
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
impl fmt::Display for Statement {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{};", match (self) {
            Statement::Declaration (declaration) => format!("{}", declaration),
            Statement::Expression  (expression)  => format!("{}", expression)
        });
    }
}


#[derive(Clone)]
pub enum Expression {

    Function(
        Vec<(String, Type)>, // Arguments
        Type,                // Return
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
    Call(
        Box<Expression>,     // Base
        Box<Vec<Expression>> // Arguments
    ),

    Integer(i64),
    VarAccess(String),
    String(String)
}
impl fmt::Display for Expression {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", match (self) {

            Expression::Function(args, ret, body) => format!(
                "(|{}| {} {{{}}})",
                args.iter().map(|(name, typ)| format!("{} : {}", name, typ)).collect::<Vec<String>>().join(", "),
                ret,
                body.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(" ")
            ),

            Expression::Addition(left, right) => format!(
                "({} + {})",
                left,
                right
            ),
            Expression::Subtraction(left, right) => format!(
                "({} - {})",
                left,
                right
            ),
            Expression::Multiplication(left, right) => format!(
                "({} * {})",
                left,
                right
            ),
            Expression::Division(left, right) => format!(
                "({} / {})",
                left,
                right
            ),

            Expression::StaticAccess(of, name) => format!(
                "{}::{}",
                of,
                name
            ),
            Expression::Call(of, args) => format!(
                "{}({})",
                of,
                args.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", ")
            ),

            Expression::Integer   (value) => format!("{}", value),
            Expression::VarAccess (name)  => format!("{}", name),
            Expression::String    (value) => format!("\"{}\"", value)

        });
    }
}

pub enum ExpressionModifier {
    StaticAccess(
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
            ExpressionModifier::Call         (args) => Expression::Call(Box::new(of), Box::new(args))
        };
    }
}


#[derive(Clone)]
pub struct Type {
    parts : Vec<String>
}
impl fmt::Display for Type {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.parts.join("::"));
    }
}



pub fn read(path : &str) -> Vec<Declaration> {
    match (parser::program(&fs::read_to_string(path).unwrap())) {

        Ok(declarations) => {
            
            println!("\n{}\n",  declarations
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join("\n")
            );
            return declarations;

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

        pub rule program() -> Vec<Declaration>
            = _ e:(_ e:declaration_with_headers() _ ";" _ {e})* _
                {e}

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
            = _ "import" _ e:ident() _
                {DeclarationType::Import(e)}
        rule declaration_initvar() -> DeclarationType
            = _ m:$("cst" / "var") _ n:ident() _ "=" _ e:expression() _
                {DeclarationType::InitVar(m == String::from("var"), n, e)}

        rule statement() -> Statement
            = _ d:declaration() _
                {Statement::Declaration(d)}
            / _ e:expression() _
                {Statement::Expression(e)}

        rule expression() -> Expression
            = _ "|" _ "|" _ r:typ() _ "{" _ s:(s:(_ s:statement() _ ";" {s}) {s})* _ "}" _
                {Expression::Function(Vec::new(), r, s)}
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
            = i:((_ i:ident() _ {i}) ** "::")
                {Type {
                    parts : i
                }}

        rule _()
            = quiet!{(" " / "\t" / "\n" / "\r")*}

    }
}
