use super::main;
use crate::types::ast::*;
use crate::types::general::*;

#[derive(Debug, Clone)]
enum AoT {
    Node(Expression),
    Token(Token),
}

pub fn parse(tokens: &[Token]) -> Result<Expression, String> {
    if tokens.len() == 1 {
        return Ok(Expression::Value(main::value(tokens[0].clone())?));
    } else {
        let nodes = paren(tokens);
        println!(r#""{:?}""#, nodes);
        let parsed = binary_operation(
            nodes?,
            &[
                &[
                    ("*", "multi", "number"),
                    ("/", "division", "number"),
                    ("%", "division_not_much", "number"),
                ],
                &[("+", "add", "number"), ("-", "remove", "number")],
                &[
                    ("==", "equal", "bool"),
                    ("!=", "n_equal", "bool"),
                    ("<", "right_big", "bool"),
                    ("<=", "maybe_right_big", "bool"),
                    (">", "left_big", "bool"),
                    (">=", "maybe_left_big", "bool"),
                ],
                &[("||", "or", "bool"), ("&&", "and", "bool")],
            ],
        )?;
        println!(r#""{:?}""#, parsed);
        assert!(parsed.len() == 1);
        for token in parsed {
            if let AoT::Node(e) = token {
                return Ok(e);
            } else if let AoT::Token(t) = token {
                return Err(format!(
                    "unknown operators: {} in {}:{} ~ {}:{}",
                    t.val,
                    tokens[0].location.start_column,
                    tokens[0].location.start_line,
                    tokens[tokens.len() - 1].location.end_column,
                    tokens[tokens.len() - 1].location.end_line
                ));
            }
        }
        return Err(String::from("unreachable error!!!"));
    }
}

fn paren(tokens: &[Token]) -> Result<Vec<AoT>, String> {
    let mut stage: u32 = 0;
    let mut before_token: Option<Token> = None;
    let mut outer: Vec<AoT> = Vec::new();
    let mut inner: Vec<Token> = Vec::new();
    for token in tokens {
        match &token.val[..] {
            "(" => {
                stage += 1;
                inner.push(token.clone());
            }
            ")" => {
                stage -= 1;
                inner.push(token.clone());
                if stage == 0 {
                    if before_token.is_none()
                        || matches!(
                            &before_token.as_ref().unwrap().val[..],
                            "+" | "-"
                                | "*"
                                | "/"
                                | "%"
                                | "=="
                                | "!="
                                | ">"
                                | "<"
                                | ">="
                                | "<="
                                | "&&"
                                | "||"
                        )
                    {
                        outer.push(AoT::Node(parse(&inner[1..inner.len() - 1])?));
                    } else {
                        outer.pop();
                        outer.push(AoT::Node(Expression::Call(main::call(
                            &crate::utils::general::flatten_vec(vec![
                                vec![before_token.unwrap()],
                                inner,
                            ]),
                        )?)));
                        before_token = None;
                    }
                    inner = Vec::new();
                };
            }
            _ => {
                if stage == 0 {
                    before_token = Some(token.clone());
                    if matches!(
                        &before_token.as_ref().unwrap().val[..],
                        "+" | "-"
                            | "*"
                            | "/"
                            | "%"
                            | "=="
                            | "!="
                            | ">"
                            | "<"
                            | ">="
                            | "<="
                            | "&&"
                            | "||"
                    ) {
                        outer.push(AoT::Token(token.clone()));
                    } else {
                        outer.push(AoT::Node(parse(&[token.clone()])?));
                    }
                } else {
                    inner.push(token.clone());
                }
            }
        }
    }
    println!("{:?}", outer);
    return Ok(outer);
}

fn binary_operation(
    tokens: Vec<AoT>,
    levels: &[&[(&str, &str, &str)]],
) -> Result<Vec<AoT>, String> {
    let mut stack2: Vec<AoT> = Vec::new();
    let mut stack: Vec<AoT> = tokens;
    for ops in levels {
        let mut result: Option<Expression> = None;
        let mut mode = "none".to_string();
        'check_a_token: for token in stack {
            if let AoT::Token(t) = token {
                for op in *ops {
                    if t.val == op.0 {
                        mode = op.1.to_string();
                        println!("{}", op.1);
                        continue 'check_a_token;
                    }
                }
                if let Some(r) = result {
                    stack2.push(AoT::Node(r));
                    result = None;
                };
                stack2.push(AoT::Token(t));
                mode = "others".to_string();
            } else if let AoT::Node(node) = token {
                if result.is_some() {
                    if mode != "others" {
                        result = Some(process(result, mode, node, "unknown")?);
                        mode = "none".to_string();
                    } else {
                        stack2.push(AoT::Node(result.unwrap()));
                        result = None;
                    }
                } else {
                    result = Some(node);
                }
            }
        }
        if let Some(r) = result {
            stack2.push(AoT::Node(r));
        }
        stack = stack2;
        stack2 = Vec::new();
    }
    return Ok(stack);
}

fn process(
    tmp: Option<Expression>,
    mode: String,
    value: Expression,
    _data_type: &'static str,
) -> Result<Expression, String> {
    return if let Option::<Expression>::Some(v) = tmp {
        Ok(Expression::Call(CallFunc {
            location: match &value {
                Expression::Call(node) => node.location.clone(),
                Expression::Value(node) => match node {
                    Value::Literal(node) => node.location.clone(),
                },
            },
            func: format!("!op_{};", mode),
            args: vec![v, value],
        }))
    } else {
        Ok(value)
    };
}
