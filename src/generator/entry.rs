use crate::type_checker::ast_types::*;

pub fn generate(ast: Vec<Component>) -> Result<String, String> {
    let mut result = String::new();
    result.push_str("\"use strict\"\n");
    for a in ast {
        result.push_str(&match a {
            Component::FnDeclar(f) => fn_declar(f)?,
            Component::RawJS(c) => c,
            Component::FnSignature(_) => "".to_string(),
        })
    }
    result.push_str("$sugot_main()");
    Ok(result)
}

fn fn_declar(node: FnDeclar) -> Result<String, String> {
    Ok(format!(
        "function $sugot_{} ({}) {{{}}}",
        node.name,
        {
            let mut s = String::new();
            for a in node.args {
                s.push_str(&format!("$sugot_{},", a.0))
            }
            s
        },
        {
            let mut s = String::new();
            for a in node.block {
                s.push_str(&statement(a)?)
            }
            s
        }
    ))
}

fn statement(node: Statement) -> Result<String, String> {
    match node {
        Statement::Block(b) => {
            let mut r = String::new();
            for a in b {
                r.push_str(&statement(a)?)
            }
            Ok(format!("{{{}}};", r))
        }
        Statement::Expression(e) => Ok(format!("{};", expression(e)?)),
        Statement::If(i) => {
            if let Some(else_block) = i.else_block {
                Ok(format!(
                    "if ({}) {{{}}} else {{{}}};",
                    expression(i.then_cond)?,
                    {
                        let mut r = String::new();
                        for a in i.then_block {
                            r.push_str(&statement(a)?)
                        }
                        r
                    },
                    {
                        let mut r = String::new();
                        for a in else_block {
                            r.push_str(&statement(a)?)
                        }
                        r
                    }
                ))
            } else {
                Ok(format!("if ({}) {{{}}};", expression(i.then_cond)?, {
                    let mut r = String::new();
                    for a in i.then_block {
                        r.push_str(&statement(a)?)
                    }
                    r
                }))
            }
        }
        Statement::VarDeclar(v) => Ok(format!("let $sugot_{} = {};", v.name, expression(v.val)?)),
        Statement::VarUpdate(v) => Ok(format!("$sugot_{} = {};", v.name, expression(v.val)?)),
        Statement::While(q) => Ok(format!("while ({}) {{{}}};", expression(q.cond)?, {
            let mut r = String::new();
            for a in q.block {
                r.push_str(&statement(a)?)
            }
            r
        })),
    }
}

fn expression(node: TypedExpression) -> Result<String, String> {
    match node.val {
        Expression::Call(c) => {
            let mut args = String::new();
            for arg in c.args {
                args.push_str(&format!("{}, ", expression(arg)?))
            }
            Ok(format!("$sugot_{}({})", c.name, args))
        }
        Expression::Literal(l) => match &l.kind[..] {
            "string" => Ok(format!("{}", l.val)),
            "int" => Ok(format!("{}", l.val)),
            "float" => Ok(format!("{}", l.val)),
            "bool" => Ok(format!("{}", l.val)),
            _ => Err(format!("unknown literal")),
        },
        Expression::Operation(o) => {
            let o = *o;
            let (l, r) = (expression(o.left)?, expression(o.right)?);
            match &o.kind[..] {
                "add" => Ok(format!("({} + {})", l, r)),
                "sub" => Ok(format!("({} - {})", l, r)),
                "mul" => Ok(format!("({} * {})", l, r)),
                "div_1" => Ok(format!("({} / {})", l, r)),
                "div_2" => Ok(format!("({} % {})", l, r)),
                "eq" => Ok(format!("({} === {})", l, r)),
                "neq" => Ok(format!("({} !== {})", l, r)),
                _ => Err(format!("unknown operator")),
            }
        }
        Expression::Variable(v) => Ok(format!("$sugot_{}", v.name)),
        Expression::Object((_, o)) => Ok(format!("{{{}}}", {
            let mut s = String::new();
            for (n, e) in o {
                s.push_str(&format!("{}: {},", n, expression(e)?))
            }
            s
        })),
        Expression::Prop((e, p)) => Ok(format!("{}.{}", expression(*e)?, p)),
    }
}
