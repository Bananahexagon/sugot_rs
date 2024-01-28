#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: String,
    pub val: String,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Operation(Box<Operation>),
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: String,
    pub left: Expression,
    pub right: Expression,
}
