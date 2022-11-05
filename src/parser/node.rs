use std::collections::HashMap;

use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub decls : Vec<Declaration>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Declaration {
    pub headers : Vec<DeclarationHeader>,
    pub vis     : DeclarationVisibility,
    pub decl    : DeclarationType
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeclarationHeader {
    Entry
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DeclarationVisibility {
    Public,
    Private
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeclarationType {
    Function(
        String,                        // Name
        Vec<(String, TypeDescriptor)>, // Arguments
        Option<TypeDescriptor>,        // Return
        Block                          // Block
    )
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Statement {
    InitVar(
        String,    // Name
        Expression // Value
    ),
    Expression(Expression)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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


#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Literal {
    Int(String),
    Float(
        String, // Integer
        String  // Decimal
    ),
    Identifier(String)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeDescriptorParts {
    BuiltIn(String),
    Custom(Vec<String>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeDescriptor {
    pub parts  : TypeDescriptorParts,
    pub constr : HashMap<String, Literal>
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub stmts   : Vec<Statement>,
    pub retlast : bool            // Return the value of the last statement
}
