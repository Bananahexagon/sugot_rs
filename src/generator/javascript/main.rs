use crate::types::ast::*;

pub fn generate(ast: Vec<FuncDeclar>) -> String {
    let mut result = String::new();
    for func in ast {
        result.push_str(&func_decl(func));
    }
    result.push_str("\nmain();");
    return result;
}

fn func_decl(declar: FuncDeclar) -> String {
    let name = declar.name;
    let mut args = String::new();
    for arg in declar.input_types {
        args.push_str(&format!("{},", arg.name))
    }
    let define = statement(declar.define);
    return format!("const {} = ({}) => {{{}}}", name, args, define);
}

fn statement(node: Statement) -> String {
    match node {
        Statement::Block(node) => block(node),
        Statement::Call(node) => call_func(node),
        Statement::VarDeclar(node) => var_decl(node), //TODO: 実装する 2023-10-06
    }
}

fn block(node: Block) -> String {
    let mut block = String::new();
    for content in node.contents {
        block.push_str(&statement(content));
    }
    return block;
}

fn call_func(node: CallFunc) -> String {
    let name = node.func;
    let mut args = String::new();
    for arg in node.args {
        args.push_str(&format!("{},", expression(arg)))
    }
    format!("{}({})", name, args)
}

fn expression(node: Expression) -> String {
    match node {
        Expression::Call(_) => unimplemented!(), //TODO: 実装する 2023-10-06
        Expression::Value(v) => value(v),
        Expression::Calc(_) => unimplemented!(), //TODO: 実装する 2023-10-07
    }
}

fn value(node: Value) -> String {
    match node {
        Value::Literal(v) => v.val,
    }
}

fn var_decl(_node: VarDeclar) -> String {
    unimplemented!() //TODO: 実装する 2023-10-06
}
