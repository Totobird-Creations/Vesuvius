#![allow(unused)]


use std::collections::HashMap;

use relative_path::RelativePathBuf;

use line_col::LineColLookup;


#[derive(Debug, Clone)]
pub struct Range(pub RelativePathBuf, pub usize, pub usize);
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
pub struct LineColumn(pub RelativePathBuf, pub (usize, usize), pub (usize, usize));


#[derive(Debug)]
pub(crate) struct Program {
    pub(crate) decls : Vec<Declaration>
}


#[derive(Debug)]
pub(crate) struct Declaration {
    pub(crate) headers : Vec<DeclarationHeader>,
    pub(crate) vis     : DeclarationVisibility,
    pub(crate) decl    : DeclarationType,
    pub(crate) range   : Range
}

#[derive(Debug)]
pub(crate) struct DeclarationHeader {
    pub(crate) header : DeclarationHeaderType,
    pub(crate) range  : Range
}
#[derive(Debug)]
pub(crate) enum DeclarationHeaderType {
    Entry
}

#[derive(Debug)]
pub(crate) struct DeclarationVisibility {
    pub(crate) vis   : DeclarationVisibilityType,
    pub(crate) range : Range
}
#[derive(Debug)]
pub(crate) enum DeclarationVisibilityType {
    Public,
    Private
}

#[derive(Debug)]
pub(crate) enum DeclarationType {
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
pub(crate) struct Statement {
    pub(crate) stmt  : StatementType,
    pub(crate) range : Range
}
#[derive(Debug, Clone)]
pub(crate) enum StatementType {
    InitVar(
        String,    // Name
        Range,     // Name Range
        Expression // Value
    ),
    Expression(Expression)
}

#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) expr  : ExpressionType,
    pub(crate) range : Range
}
#[derive(Debug, Clone)]
pub(crate) enum ExpressionType {

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
pub(crate) struct Atom {
    pub(crate) atom  : AtomType,
    pub(crate) range : Range
}
#[derive(Debug, Clone)]
pub(crate) enum AtomType {
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
pub(crate) struct Literal {
    pub(crate) lit   : LiteralType,
    pub(crate) range : Range
}
#[derive(Debug, Clone)]
pub(crate) enum LiteralType {
    Int(String),
    Float(
        String, // Integer
        String  // Decimal
    ),
    Identifier(String)
}


#[derive(Debug, Clone)]
pub(crate) enum TypeDescriptorParts {
    BuiltIn(String),
    Custom(Vec<String>)
}

#[derive(Debug, Clone)]
pub(crate) struct TypeDescriptor {
    pub(crate) parts  : TypeDescriptorParts,
    pub(crate) constr : HashMap<String, Literal>
}


#[derive(Clone)]
pub(crate) struct Block {
    pub(crate) stmts   : Vec<Statement>,
    pub(crate) retlast : bool,           // Return the value of the last statement
    pub(crate) range   : Range
}
