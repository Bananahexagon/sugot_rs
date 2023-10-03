use super::general::Location;

pub struct FuncDeclar {
    pub location: Location,
    pub name: String,
    pub input_types: Vec<VarDeclar>,
    pub return_type: DataType,
    pub define: Statement,
}

pub struct VarDeclar {
    pub location: Location,
    pub name: String,
    pub data_type: DataType,
    pub init: Option<Expression>,
}

pub struct DataType {
    pub location: Location,
    pub value: String,
}
pub enum Statement {
    Block(Block),
    VarDeclar(VarDeclar),
    Call(CallFunc),
}

pub struct Block {
    pub location: Location,
    pub contents: Vec<Statement>,
}

pub enum Expression {
    Call(CallFunc),
}

pub struct CallFunc {
    pub location: Location,
    pub func: String,
    pub args: Vec<Expression>,
}
