use std::collections::HashMap;

use serde::{
    Serialize,
    Deserialize
};
use line_col::LineColLookup;


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Range(pub usize, pub usize);
impl Range {
    pub fn to_linecolumn(&self, script : &String) -> LineColumn {
        let lookup = LineColLookup::new(script);
        return LineColumn(
            lookup.get(self.0),
            lookup.get(self.1)
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct LineColumn(pub (usize, usize), pub (usize, usize));


#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub decls : Vec<Declaration>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Declaration {
    pub headers : Vec<DeclarationHeader>,
    pub vis     : DeclarationVisibility,
    pub decl    : DeclarationType,
    pub range   : Range
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationHeader {
    pub header : DeclarationHeaderType,
    pub range  : Range
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeclarationHeaderType {
    Entry
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationVisibility {
    pub vis   : DeclarationVisibilityType,
    pub range : Range
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeclarationVisibilityType {
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
pub struct Statement {
    pub stmt  : StatementType,
    pub range : Range
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StatementType {
    InitVar(
        String,    // Name
        Expression // Value
    ),
    Expression(Expression)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expression {
    pub expr  : ExpressionType,
    pub range : Range
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExpressionType {

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
pub struct Atom {
    pub atom  : AtomType,
    pub range : Range
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AtomType {
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
    pub retlast : bool,           // Return the value of the last statement
    pub range   : Range
}
