use super::general::Location;
#[derive(Debug, Clone)]
pub struct FuncDeclar {
    pub location: Location,
    pub name: String,
    pub input_types: Vec<FuncArgs>,
    pub return_type: DataType,
    pub define: Block,
}

#[derive(Debug, Clone)]
pub struct FuncArgs {
    pub location: Location,
    pub name: String,
    pub data_type: DataType,
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
pub struct VarSet {
    pub location: Location,
    pub name: String,
    pub val: Expression,
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
    VarSet(VarSet),
    Call(CallFunc),
    Return(Expression),
    If(If),
    While(While),
}
#[derive(Debug, Clone)]
pub struct If {
    pub location: Location,
    pub condition:Expression,
    pub then_contents: Block,
    pub else_contents: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub location: Location,
    pub condition:Expression,
    pub contents: Block,
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
