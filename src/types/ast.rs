use super::general::Location;

pub struct Program {
    pub location: Location,
    pub body: FnDeclar,
}

pub struct FnDeclar {
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
    pub define: Option<Expression>,
}

pub struct DataType {
    pub location: Location,
    pub value: String,
}

pub struct Statement {
    pub location: Location,
    pub exp: Expression,
}

pub struct Expression {}
