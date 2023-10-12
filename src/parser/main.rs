use crate::types::ast::*;
use crate::types::general::*;

use super::expression;

pub fn main(tokens: Vec<Token>) -> Result<Vec<FuncDeclar>, String> {
    let mut stage = 0;
    let mut decl = Vec::new();
    let mut result = Vec::<FuncDeclar>::new();
    for token in tokens {
        if matches!(&token.val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&token.val[..], ")" | "}") {
            stage -= 1;
        } else if &token.val == "fn" && stage == 0 && !decl.is_empty() {
            result.push(func_decl(&decl)?);
            decl = Vec::new();
        };
        decl.push(token);
    }
    if stage == 0 {
        if !decl.is_empty() {
            result.push(func_decl(&decl)?);
        }
    } else {
        panic!();
    }
    return Ok(result);
}

fn func_decl(tokens: &[Token]) -> Result<FuncDeclar, String> {
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
    return Ok(FuncDeclar {
        location: location,
        name: name,
        input_types: args, //TODO 今は入力を取らない関数だけ 2023-10-03
        return_type: data_type(return_type)?,
        define: statement(&define)?,
    });
}

fn statement(tokens: &[Token]) -> Result<Statement, String> {
    Ok(match &(tokens[0].val[..]) {
        "{" => Statement::Block(block(tokens)?),
        "let" | "var" => Statement::VarDeclar(var_declar(tokens)?),
        "return" => Statement::Return(expression::parse(&tokens[1..])?),
        _ => Statement::Call(call(tokens)?),
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
    return Ok(VarDeclar {
        location,
        name: var_name,
        data_type: data_type(data_type_tokens[0].clone())?, //TODO: ちゃんとやる 2023-10-07
        init,
        is_mut,
    });
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
        location: location,
        func: func_name,
        args: args,
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
    return Ok(Block {
        location: location,
        contents: result,
    });
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
