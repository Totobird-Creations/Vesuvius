use std::collections::HashMap;


#[derive(Debug)]
pub struct Program {
    pub decls : Vec<Declaration>
}


#[derive(Debug)]
pub struct Declaration {
    pub headers : Vec<DeclarationHeader>,
    pub vis     : DeclarationVisibility,
    pub decl    : DeclarationType
}
#[derive(Debug)]
pub enum DeclarationHeader {
    Entry
}
#[derive(Debug, Copy, Clone)]
pub enum DeclarationVisibility {
    Public,
    Private
}
#[derive(Debug)]
pub enum DeclarationType {
    Function(
        String,              // Name
        Vec<(String, Type)>, // Arguments
        Option<Type>,        // Return
        Block                // Block
    )
}


#[derive(Debug, Clone)]
pub enum Statement {
    InitVar(
        String,    // Name
        Expression // Value
    ),
    Expression(Expression)
}

#[derive(Debug, Clone)]
pub enum Expression {

    EqualsOperation(Box<Expression>, Box<Expression>),
    NotEqualsOperation(Box<Expression>, Box<Expression>),
    GreaterOperation(Box<Expression>, Box<Expression>),
    GreaterEqualsOperation(Box<Expression>, Box<Expression>),
    LessOperation(Box<Expression>, Box<Expression>),
    LessEqualsOperation(Box<Expression>, Box<Expression>),
    AdditionOperation(Box<Expression>, Box<Expression>),
    SubtractionOperation(Box<Expression>, Box<Expression>),
    MultiplicationOperation(Box<Expression>, Box<Expression>),
    DivisionOperation(Box<Expression>, Box<Expression>),

    Atom(Atom)
}


#[derive(Debug, Clone)]
pub enum Atom {
    Literal(Literal),
    Expression(Box<Expression>),
    If(
        Vec<(
            Box<Expression>, // Condition
            Block            // Block
        )>,
        Option<Block>        // Else block
    )
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(String),
    Float(
        String, // Integer
        String  // Decimal
    ),
    Identifier(String)
}


#[derive(Debug, Clone)]
pub struct Type {
    parts  : Vec<String>,
    constr : HashMap<String, Literal>
}


#[derive(Clone)]
pub struct Block {
    pub stmts   : Vec<Statement>,
    pub retlast : bool            // Return the value of the last statement
}
