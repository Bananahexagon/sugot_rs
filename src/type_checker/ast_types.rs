use std::collections::HashMap;
use crate::ast_types::DataType;

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: String,
    pub val: String,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Operation(Box<Operation>),
    Call(Call),
    Variable(Variable),
    Object((String, HashMap<String, TypedExpression>)),
    Prop((Box<TypedExpression>, String)),
    Cast((Box<TypedExpression>, DataType))
}

#[derive(Debug, Clone)]
pub struct TypedExpression {
    pub val: Expression,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub enum Component {
    FnDeclar(FnDeclar),
    RawJS(String),
    FnSignature(FnSignature),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(TypedExpression),
    VarDeclar(VarDeclar),
    VarUpdate(VarUpdate),
    Block(Vec<Statement>),
    If(If),
    While(While)
}

#[derive(Debug, Clone)]
pub struct While {
    pub cond: TypedExpression,
    pub block: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub then_cond: TypedExpression,
    pub then_block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>
}

#[derive(Debug, Clone)]
pub struct FnDeclar {
    pub name: String,
    pub args: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub block: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct FnSignature {
    pub name: String,
    pub args: Vec<(String, DataType)>,
    pub return_type: DataType,
}

#[derive(Debug, Clone)]
pub struct VarDeclar {
    pub name: String,
    pub val: TypedExpression,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub struct VarUpdate {
    pub name: String,
    pub val: TypedExpression
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: String,
    pub args: Vec<TypedExpression>
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: String,
    pub left:  TypedExpression,
    pub right: TypedExpression,
}