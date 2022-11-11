use std::{
    collections::HashMap,
    path::PathBuf
};

use line_col::LineColLookup;


#[derive(Debug, Clone)]
pub struct Range(pub PathBuf, pub usize, pub usize);
impl Range {
    pub fn to_linecolumn(&self, script : &String) -> LineColumn {
        let lookup = LineColLookup::new(script);
        return LineColumn(
            self.0.clone(),
            lookup.get(self.1),
            lookup.get(self.2)
        );
    }
}

#[derive(Debug, Clone)]
pub struct LineColumn(pub PathBuf, pub (usize, usize), pub (usize, usize));


#[derive(Debug)]
pub struct Program {
    pub decls : Vec<Declaration>
}


#[derive(Debug)]
pub struct Declaration {
    pub headers : Vec<DeclarationHeader>,
    pub vis     : DeclarationVisibility,
    pub decl    : DeclarationType,
    pub range   : Range
}

#[derive(Debug)]
pub struct DeclarationHeader {
    pub header : DeclarationHeaderType,
    pub range  : Range
}
#[derive(Debug)]
pub enum DeclarationHeaderType {
    Entry
}

#[derive(Debug)]
pub struct DeclarationVisibility {
    pub vis   : DeclarationVisibilityType,
    pub range : Range
}
#[derive(Debug)]
pub enum DeclarationVisibilityType {
    Public,
    Private
}

#[derive(Debug)]
pub enum DeclarationType {
    Module(
        Vec<String>,
        Range
    ),
    Function(
        String,                        // Name
        Range,                         // Name Range
        Vec<(String, TypeDescriptor)>, // Arguments
        Option<TypeDescriptor>,        // Return
        Block                          // Block
    )
}


#[derive(Debug, Clone)]
pub struct Statement {
    pub stmt  : StatementType,
    pub range : Range
}
#[derive(Debug, Clone)]
pub enum StatementType {
    InitVar(
        String,    // Name
        Range,     // Name Range
        Expression // Value
    ),
    Expression(Expression)
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr  : ExpressionType,
    pub range : Range
}
#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub struct Atom {
    pub atom  : AtomType,
    pub range : Range
}
#[derive(Debug, Clone)]
pub enum AtomType {
    Literal(Literal),
    Expression(Box<Expression>),
    If(
        Vec<(
            Box<Expression>, // Condition
            Block,           // Block
            Range
        )>,
        Option<(
            Block, // Else block
            Range
        )>
    )
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub lit   : LiteralType,
    pub range : Range
}
#[derive(Debug, Clone)]
pub enum LiteralType {
    Int(String),
    Float(
        String, // Integer
        String  // Decimal
    ),
    Identifier(String)
}


#[derive(Debug, Clone)]
pub enum TypeDescriptorParts {
    BuiltIn(String),
    Custom(Vec<String>)
}

#[derive(Debug, Clone)]
pub struct TypeDescriptor {
    pub parts  : TypeDescriptorParts,
    pub constr : HashMap<String, Literal>
}


#[derive(Clone)]
pub struct Block {
    pub stmts   : Vec<Statement>,
    pub retlast : bool,           // Return the value of the last statement
    pub range   : Range
}
