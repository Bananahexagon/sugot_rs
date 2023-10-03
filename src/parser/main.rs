use crate::types::ast::FuncDeclar;
use crate::types::general::Location;
use crate::types::general::Token;

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
    return FuncDeclar {
        location: location,
        name: name,
        input_types: vec![] , //TODO 今は入力を取らない関数だけ
        return_type: unimplemented!(), 
        define: unimplemented!(),
    }
}
