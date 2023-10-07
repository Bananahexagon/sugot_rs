use super::main;
use crate::types::ast::*;
use crate::types::general::*;

#[derive(Debug, Clone)]
enum Aos {
    Node(Expression),
    Token(Token),
}

pub fn parse(tokens: &[Token]) -> Expression {
    if tokens.len() == 1 {
        return Expression::Value(main::value(tokens[0].clone()));
    } else {
        let nodes = paren(tokens);
        if nodes.len() == 1 {
            match nodes[0].clone() {
                Aos::Node(n) => return n,
                Aos::Token(_) => panic!(),
            }
        }
        unimplemented!();
    }
}

fn paren(tokens: &[Token]) -> Vec<Aos> {
    let mut stage: u32 = 0;
    let mut before_token: Option<Token> = None;
    let mut outer: Vec<Aos> = Vec::new();
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
                            "+" | "-" | "*" | "/"
                        )
                    {
                        outer.push(Aos::Node(parse(&inner[1..inner.len() - 1])));
                    } else {
                        outer.pop();
                        outer.push(Aos::Node(Expression::Call(main::call(
                            &crate::utils::general::flatten_vec(vec![
                                vec![before_token.unwrap()],
                                inner,
                            ]),
                        ))));
                        before_token = None;
                    }
                    inner = Vec::new();
                };
            }
            _ => {
                if stage == 0 {
                    before_token = Some(token.clone());
                    outer.push(Aos::Token(token.clone()));
                } else {
                    inner.push(token.clone());
                }
            }
        }
    }
    println!("{:?}", outer);
    return outer;
}
