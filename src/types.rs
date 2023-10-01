pub enum ASTNode {
    FnDeclar(FnDeclar),
    VaeDeclar(VarDeclar),
}

struct FnDeclar {
    name: String,
    input_types: Vec<DataType>,
    return_type: DataType,
    define: Statement,
}

struct VarDeclar {
    name: String,
    data_type: DataType,
    define: Option<Expression>,
}

struct DataType {
    value: String,
}

struct Statement {
    exp: Expression,
}

struct Expression {}
