pub enum ASTNode {}

struct FnDeclar {
    input_types: Vec<DataType>,
    return_type: DataType,
    define: Statement,
}

struct DataType {
    value: String,
}

struct Statement {
    
}