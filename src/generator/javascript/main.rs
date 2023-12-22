use crate::types::ast::*;

use super::libjs;

pub fn generate(ast: Vec<FuncDeclar>) -> String {
    let mut result = String::new();
    result.push_str(libjs::io::CODE);
    for func in ast {
        result.push_str(&func_decl(func));
    }
    result.push_str("\nmain();");
    result
}

fn func_decl(declar: FuncDeclar) -> String {
    let name = declar.name;
    let mut args = String::new();
    for arg in declar.input_types {
        args.push_str(&format!("{},", arg.name))
    }
    let define =block(declar.define);
    format!("function {} ({}) {{{}}};", name, args, define)
}

fn statement(node: Statement) -> String {
    match node {
        Statement::Block(node) => block(node),
        Statement::Call(node) => format!("{};", call_func(node)),
        Statement::VarDeclar(node) => var_decl(node),
        Statement::VarSet(node) => var_set(node),
        Statement::Return(node) => format!("return {};", expression(node)),
        Statement::If(node) => format!("{};", if_statement(node)),
        Statement::While(node) => format!("{};", while_statement(node)),
    }
}

fn if_statement(node: If) -> String {
    if node.else_contents.is_none() {
        format!(
            "if ({}) {{{}}}",
            expression(node.condition),
            block(node.then_contents)
        )
    } else {
        format!(
            "if ({}) {{{}}} else {{{}}}",
            expression(node.condition),
            block(node.then_contents),
            block(node.else_contents.unwrap())
        )
    }
}
fn while_statement(node: While) -> String {
    format!(
        "while ({}) {{{}}}",
        expression(node.condition),
        block(node.contents)
    )
}

fn block(node: Block) -> String {
    let mut block = String::new();
    for content in node.contents {
        block.push_str(&statement(content));
    }
    block
}

fn call_func(node: CallFunc) -> String {
    let name = node.func;
    if &name[0..1] == "!" {
        let mut i = 1;
        let mut call_type = String::new();
        while !matches!(&name[i..i + 1], "_" | ";") {
            call_type.push_str(&name[i..i + 1]);
            i += 1;
        }
        let rest = &name[i + 1..name.len() - 1];
        if &name[i..i + 1] == "_" {
            match &call_type[..] {
                "op" => format!(
                    "({0} {2} {1})",
                    expression(node.args[0].clone()),
                    expression(node.args[1].clone()),
                    match rest {
                        "multi" => "*",
                        "division" => "/",
                        "division_not_much" => "%",
                        "add" => "+",
                        "remove" => "-",
                        "equal" => "===",
                        "n_equal" => "!==",
                        "right_big" => "<",
                        "maybe_right_big" => "<=",
                        "left_big" => ">",
                        "maybe_left_big" => ">=",
                        "or" => "||",
                        "and" => "&&",
                        _ => unimplemented!(),
                    }
                ),
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    } else {
        let mut args = String::new();
        for arg in node.args {
            args.push_str(&format!("{},", expression(arg)))
        }
        format!("{}({})", name, args)
    }
}

fn expression(node: Expression) -> String {
    match node {
        Expression::Call(c) => call_func(c), //TODO: 実装する 2023-10-06
        Expression::Value(v) => value(v),
    }
}

fn value(node: Value) -> String {
    match node {
        Value::Literal(v) => v.val,
    }
}

fn var_decl(node: VarDeclar) -> String {
    if node.init.is_some() {
        format!(
            "{} {} = {};",
            if node.is_mut { "let" } else { "const" },
            node.name,
            expression(node.init.unwrap())
        )
    } else {
        format!(
            "{} {};",
            if node.is_mut { "let" } else { "const" },
            node.name,
        )
    }
}

fn var_set(node: VarSet) -> String {
    format!("{} = {};", node.name, expression(node.val))
}
