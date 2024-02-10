use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: String,
    pub val: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Operation(Box<Operation>),
    Call(Call),
    Variable(Variable),
    Object((String, HashMap<String, Expression>)),
    Prop((Box<Expression>, String)),
    Cast((Box<Expression>, DataType)),
    Index((Box<Expression>, Box<Expression>))
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Name(String),
    Object(HashMap<String,DataType>),
    Array(Box<DataType>),
}

pub enum Component {
    FnDeclar(FnDeclar),
    RawJS(String),
    FnSignature(FnSignature)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VarDeclar(VarDeclar),
    VarUpdate(VarUpdate),
    Block(Vec<Statement>),
    If(If),
    While(While)
}

#[derive(Debug, Clone)]
pub struct While {
    pub cond: Expression,
    pub block: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub then_cond: Expression,
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
    pub val: Expression,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub struct VarUpdate {
    pub name: String,
    pub val: Expression
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expression>
}

impl Call {
    pub fn into_expression(self) -> Expression {
        Expression::Call(self)
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: String,
    pub left: Expression,
    pub right: Expression,
}

impl Operation {
    pub fn into_expression(self) -> Expression {
        Expression::Operation(Box::new(self))
    }
}