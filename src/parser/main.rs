use std::collections::HashSet;

use crate::types::ast::*;
use crate::types::general::*;

use super::expression;

pub fn main(tokens: Vec<Token>) -> Result<Vec<Declars>, String> {
    let mut stage = 0;
    let mut decl: Vec<Token> = Vec::new();
    let mut result = Vec::new();
    for token in tokens {
        if matches!(&token.val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&token.val[..], ")" | "}") {
            stage -= 1;
        } else if matches!(&token.val[..], "fn" | "import") && stage == 0 && !decl.is_empty() {
            result.push(match &decl[0].val[..] {
                "fn" => Declars::Func(func_declar(&decl)?),
                "import" => Declars::Import(import_statement(&decl)?),
                _ => return Err(format!("TODO")) //TODO 2024-01-26
            });
            decl = Vec::new();
        };
        decl.push(token);
    }
    if stage == 0 {
        if !decl.is_empty() {
            result.push(match &decl[0].val[..] {
                "fn" => Declars::Func(func_declar(&decl)?),
                "import" => Declars::Import(import_statement(&decl)?),
                _ => return Err(format!("TODO")) //TODO 2024-01-26
            });
        }
    } else {
        panic!();
    }
    Ok(result)
}
fn import_statement(tokens: &[Token]) -> Result<Import, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut contents = HashSet::new();
    let mut path = Vec::new();
    let mut ptr = 0;
    assert_eq!(tokens[ptr].val, "import");
    ptr += 1;
    assert_eq!(tokens[ptr].val, "{");
    ptr += 1;
    while tokens[ptr].val != "}" {
        if (tokens[ptr].val != ",") == (ptr%2==1) {
            return Err(format!("TODO")) //TODO 2024-01-26
        } else if tokens[ptr].val != "," {
            contents.insert(tokens[ptr].val.clone());
        }
        ptr += 1;
    }
    assert_eq!(tokens[ptr].val, "}");
    ptr += 1;
    assert_eq!(tokens[ptr].val, "from");
    ptr += 1;
    while ptr < tokens.len() {
        path.push(tokens[ptr].val.clone());
        ptr+=1;
    }
    return Ok(Import { location, contents, path })
}
fn func_declar(tokens: &[Token]) -> Result<FuncDeclar, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    assert_eq!(tokens[ptr].val, "fn");
    ptr += 1;
    let name = tokens[1].val.clone();
    ptr += 1;
    assert_eq!(tokens[ptr].val, "(");
    ptr += 1;
    let mut args = Vec::<FuncArgs>::new();
    let mut stage = 1;
    let mut current_arg: Vec<Token> = Vec::new();
    while stage != 0 && ptr < tokens.len() {
        println!("{:?}", tokens[ptr]);
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        if stage != 0 {
            if tokens[ptr].val == "," && stage == 1 {
                args.push(FuncArgs {
                    location: Location {
                        start_line: current_arg[0].location.start_line,
                        start_column: current_arg[0].location.start_column,
                        end_line: current_arg[current_arg.len() - 1].location.end_line,
                        end_column: current_arg[current_arg.len() - 1].location.end_column,
                    },
                    name: current_arg[0].val.clone(),
                    data_type: data_type(current_arg[2].clone())?,
                });
            } else {
                current_arg.push(tokens[ptr].clone());
            };
            ptr += 1;
        }
    }
    if !current_arg.is_empty() {
        args.push(FuncArgs {
            location: Location {
                start_line: current_arg[0].location.start_line,
                start_column: current_arg[0].location.start_column,
                end_line: current_arg[current_arg.len() - 1].location.end_line,
                end_column: current_arg[current_arg.len() - 1].location.end_column,
            },
            name: current_arg[0].val.clone(),
            data_type: data_type(current_arg[2].clone())?,
        });
    }
    assert_eq!(tokens[ptr].val, ")");
    ptr += 1;
    assert_eq!(tokens[ptr].val, "->");
    ptr += 1;
    let return_type = tokens[ptr].clone();
    ptr += 1;
    let mut define = Vec::new();
    while ptr < tokens.len() {
        define.push(tokens[ptr].clone());
        ptr += 1;
    }
    Ok(FuncDeclar {
        location,
        name,
        input_types: args, //TODO 今は入力を取らない関数だけ 2023-10-03
        return_type: data_type(return_type)?,
        define: block(&define)?,
    })
}

fn statement(tokens: &[Token]) -> Result<Statement, String> {
    Ok(match &(tokens[0].val[..]) {
        "{" => Statement::Block(block(tokens)?),
        "let" | "var" => Statement::VarDeclar(var_declar(tokens)?),
        "return" => Statement::Return(expression::parse(&tokens[1..])?),
        "if" => Statement::If(if_statement(tokens)?),
        "while" => Statement::While(while_statement(tokens)?),
        _ => match &(tokens[1].val[..]) {
            "=" => Statement::VarSet(var_set(tokens)?),
            "(" => Statement::Call(call(tokens)?),
            _ => return Err("".to_string()),
        },
    })
}

fn var_declar(tokens: &[Token]) -> Result<VarDeclar, String> {
    println!("{:?}", tokens);
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    let is_mut = &tokens[ptr].val == "var";
    ptr += 1;
    let var_name = tokens[ptr].val.clone();
    ptr += 1;
    assert_eq!(tokens[ptr].val, ":");
    ptr += 1;
    let mut data_type_tokens = Vec::new();
    while tokens[ptr].val != "=" && ptr < tokens.len() {
        data_type_tokens.push(tokens[ptr].clone());
        ptr += 1;
    }
    let mut init = None;
    if tokens[ptr].val == "=" {
        ptr += 1;
        let mut init_tokens = Vec::new();
        while ptr < tokens.len() {
            init_tokens.push(tokens[ptr].clone());
            ptr += 1;
        }
        init = Some(expression::parse(&init_tokens)?);
    }
    Ok(VarDeclar {
        location,
        name: var_name,
        data_type: data_type(data_type_tokens[0].clone())?, //TODO: ちゃんとやる 2023-10-07
        init,
        is_mut,
    })
}

fn var_set(tokens: &[Token]) -> Result<VarSet, String> {
    println!("{:?}", tokens);
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    let var_name = tokens[ptr].val.clone();
    ptr += 1;
    assert_eq!(tokens[ptr].val, "=");
    ptr += 1;
    let mut val_tokens = Vec::new();
    while ptr < tokens.len() {
        val_tokens.push(tokens[ptr].clone());
        ptr += 1;
    }
    let val = expression::parse(&val_tokens)?;

    Ok(VarSet {
        location,
        name: var_name,
        val,
    })
}

pub fn call(tokens: &[Token]) -> Result<CallFunc, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    let func_name = tokens[ptr].val.clone();
    ptr += 1;
    assert_eq!(tokens[ptr].val, "(");
    ptr += 1;
    let mut args = Vec::new();
    let mut stage = 1;
    let mut current_arg = Vec::new();
    while stage != 0 && ptr < tokens.len() {
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        if stage != 0 {
            if tokens[ptr].val == "," && stage == 1 {
                args.push(expression::parse(&current_arg)?);
            } else {
                current_arg.push(tokens[ptr].clone());
            }
            ptr += 1;
        }
    }
    if !current_arg.is_empty() {
        args.push(expression::parse(&current_arg)?);
    }
    assert_eq!(tokens[ptr].val, ")");
    Ok(CallFunc {
        location,
        func: func_name,
        args,
    })
}

fn block(tokens: &[Token]) -> Result<Block, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut result = Vec::new();
    let mut current_statement = Vec::new();
    let mut stage = 0;
    let mut i = 0;
    let len = tokens.len();
    for token in tokens {
        if i == 0 || i == len - 1 {
            i += 1;
            continue;
        }
        match &token.val[..] {
            "(" | "{" => {
                stage += 1;
                current_statement.push(token.clone());
            }
            ")" | "}" => {
                stage -= 1;
                current_statement.push(token.clone());
            }
            ";" if stage == 0 => {
                result.push(statement(&current_statement)?);
                current_statement = Vec::new();
            }
            _ => current_statement.push(token.clone()),
        }
        i += 1;
    }
    Ok(Block {
        location,
        contents: result,
    })
}

pub fn value(token: Token) -> Result<Value, String> {
    Ok(Value::Literal(literal(token)?)) // TODO: ちゃんとやる 2023-10-05
}

fn literal(token: Token) -> Result<Literal, String> {
    Ok(Literal {
        location: token.location,
        val: token.val,
    })
}

fn data_type(token: Token) -> Result<DataType, String> {
    Ok(DataType {
        location: token.location,
        val: token.val,
    }) // TODO: ちゃんとやる 2023-10-05
}

fn if_statement(tokens: &[Token]) -> Result<If, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    assert_eq!(tokens[ptr].val, "if");
    ptr += 1;
    let mut stage = 0;
    let mut condition = Vec::new();
    while !(tokens[ptr].val == "{" && stage == 0) {
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        condition.push(tokens[ptr].clone());
        ptr += 1;
    }

    let mut then_contents = Vec::new();
    while !(tokens[ptr - 1].val == "}" && stage == 0) {
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        then_contents.push(tokens[ptr].clone());
        ptr += 1;
    }
    if !(ptr < tokens.len() && tokens[ptr].val == "else") {
        Ok(If {
            location,
            condition: expression::parse(&condition)?,
            then_contents: block(&then_contents)?,
            else_contents: None,
        })
    } else if ptr + 1 < tokens.len() && tokens[ptr].val == "else" {
        ptr += 1;
        if tokens[ptr].val == "if" {
            Ok(If {
                location,
                condition: expression::parse(&condition)?,
                then_contents: block(&then_contents)?,
                else_contents: Some({
                    let tmp = if_statement(&tokens[ptr..])?;
                    Block {
                        location: tmp.location.clone(),
                        contents: vec![Statement::If(tmp)],
                    }
                }),
            })
        } else {
            let mut else_contents = Vec::new();
            while !(tokens[ptr - 1].val == "}" && stage == 0) {
                if matches!(&tokens[ptr].val[..], "(" | "{") {
                    stage += 1;
                } else if matches!(&tokens[ptr].val[..], ")" | "}") {
                    stage -= 1;
                }
                else_contents.push(tokens[ptr].clone());
                ptr += 1;
            }
            Ok(If {
                location,
                condition: expression::parse(&condition)?,
                then_contents: block(&then_contents)?,
                else_contents: Some(block(&else_contents)?),
            })
        }
    } else {
        Err(format!("unxepected token '{}'", tokens[ptr].val))
    }
}

fn while_statement(tokens: &[Token]) -> Result<While, String> {
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    assert_eq!(tokens[ptr].val, "while");
    ptr += 1;
    let mut stage = 0;
    let mut condition = Vec::new();
    while !(tokens[ptr].val == "{" && stage == 0) {
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        condition.push(tokens[ptr].clone());
        ptr += 1;
    }
    assert_eq!(tokens[ptr].val, "{");
    let mut contents = Vec::new();
    while !(tokens[ptr - 1].val == "}" && stage == 0) {
        if matches!(&tokens[ptr].val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&tokens[ptr].val[..], ")" | "}") {
            stage -= 1;
        }
        contents.push(tokens[ptr].clone());
        ptr += 1;
    }
    println!("W:{:?}", contents);
    Ok(While {
        location,
        condition: expression::parse(&condition)?,
        contents: block(&contents)?,
    })
}
