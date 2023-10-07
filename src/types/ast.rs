use super::general::Location;
#[derive(Debug, Clone)]
pub struct FuncDeclar {
    pub location: Location,
    pub name: String,
    pub input_types: Vec<VarDeclar>,
    pub return_type: DataType,
    pub define: Statement,
}

#[derive(Debug, Clone)]
pub struct VarDeclar {
    pub location: Location,
    pub name: String,
    pub data_type: DataType,
    pub init: Option<Expression>,
    pub is_mut: bool,
}

#[derive(Debug, Clone)]
pub struct DataType {
    pub location: Location,
    pub val: String,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Block),
    VarDeclar(VarDeclar),
    Call(CallFunc),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub location: Location,
    pub contents: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Call(CallFunc),
    Value(Value),
    Calc(Calc),
}

#[derive(Debug, Clone)]
pub struct Calc {
    pub location: Location,
    pub op: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}

#[derive(Debug, Clone)]
pub struct CallFunc {
    pub location: Location,
    pub func: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
}
#[derive(Debug, Clone)]
pub struct Literal {
    pub location: Location,
    pub val: String,
}
