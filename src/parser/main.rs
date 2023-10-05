use crate::types::ast::*;
use crate::types::general::*;

pub fn main(tokens: Vec<Token>) -> Vec<FuncDeclar> {
    let mut stage = 0;
    let mut decl = Vec::new();
    let mut result = Vec::<FuncDeclar>::new();
    for token in tokens {
        if matches!(&token.val[..], "(" | "{") {
            stage += 1;
        } else if matches!(&token.val[..], ")" | "}") {
            stage -= 1;
        } else if &token.val == "fun" && stage == 0 {
            result.push(func_decl(decl));
            decl = Vec::new();
        };
        decl.push(token);
    }
    unimplemented!()
}

fn func_decl(tokens: Vec<Token>) -> FuncDeclar {
    println!("{:?}", tokens);
    let location = Location {
        start_line: tokens[0].location.start_line,
        start_column: tokens[0].location.start_column,
        end_line: tokens[tokens.len() - 1].location.end_line,
        end_column: tokens[tokens.len() - 1].location.end_column,
    };
    let mut ptr = 0;
    assert_eq!(tokens[ptr].val, "fun");
    ptr += 1;
    let name = tokens[1].val.clone();
    ptr += 1;
    assert_eq!(tokens[ptr].val, "(");
    ptr += 1;
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
    return FuncDeclar {
        location: location,
        name: name,
        input_types: vec![], //TODO 今は入力を取らない関数だけ 2023-10-03
        return_type: data_type(return_type),
        define: statement(define),
    };
}

fn statement(tokens: Vec<Token>) -> Statement {
    match &(tokens[0].val[..]) {
        "{" => Statement::Block(block(tokens)),
        "let" => Statement::VarDeclar(var_declar(tokens)),
        _ => Statement::Call(call(tokens)),
    }
}

fn var_declar(tokens: Vec<Token>) -> VarDeclar {
    unimplemented!() //TODO: 変数宣言をパースできるようにする 2023-10-05
}

fn call(tokens: Vec<Token>) -> CallFunc {
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
    while tokens[ptr].val != ")" && ptr < tokens.len() {
        if tokens[ptr].val != "," {
            args.push(Expression::Value(value(tokens[ptr].clone())));
        }
        ptr += 1;
    }
    assert_eq!(tokens[ptr].val, ")");
    CallFunc {
        location:location,
        func: func_name,
        args: args,
    } //TODO: 引数をちゃんとする 2023-10-05
}

fn block(tokens: Vec<Token>) -> Block {
    unimplemented!() //TODO: ブロックをパースできるようにする 2023-10-05
}

fn value(token: Token) -> Value {
    Value::Literal(literal(token)) // TODO: ちゃんとやる 2023-10-05
}

fn literal(token: Token) -> Literal {
    Literal {
        location: token.location,
        val: token.val,
    }
}

fn data_type(token: Token) -> DataType {
    DataType { location: token.location, val: token.val } // TODO: ちゃんとやる 2023-10-05
}