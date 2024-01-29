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
    Variable(Variable)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VarDeclar(VarDeclar),
    VarUpdate(VarUpdate),
}

#[derive(Debug, Clone)]
pub struct VarDeclar {
    pub name: String,
    pub val: Expression
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