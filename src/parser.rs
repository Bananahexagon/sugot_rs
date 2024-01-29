use crate::ast_types::*;

peg::parser! {
pub grammar parser() for str {

rule _ =  (" " / "\n" / "\t" / "\r")*

rule integer() -> Literal
    = n: $(['0'] / (['1'..='9']['0'..='9']*)) {Literal { kind: "integer".to_string(), val: n.to_string() } }

rule float() -> Literal
= n: $(['0'] / ['1'..='9']['0'..='9']*"."['0'..='9']) {Literal { kind: "float".to_string(), val: n.to_string() } }

rule string() -> Literal
    = s: $("\"" (!"\"" [_])* "\"") {Literal { kind: "string".to_string(), val: s.to_string() } }

rule bool() -> Literal
    = s: $("true" / "false") {Literal { kind: "bool".to_string(), val: s.to_string() } }

rule literal() -> Literal = integer() / string() / bool()

rule add_operator() -> String = s: $("+" / "-") { match s { "+" => "add", "-" => "sub",_=>unreachable!() }.to_string() }

rule add_operation() -> Expression
    = l: mul_operation() _ op: add_operator() _ r: add_operation() { Operation { kind: op ,left:l, right:r }.into_expression() }
    / mul_operation()

rule mul_operator() -> String = s: $("*" / "/" / "%") { match s { "*" => "mul", "/" => "div_1", "%" => "div_2",_=>unreachable!() }.to_string() }

rule mul_operation() -> Expression
    = l: expression_atom() _ op: mul_operator() _ r: mul_operation() { Operation { kind: op ,left:l, right:r }.into_expression() }
    / expression_atom()

rule expression_atom() -> Expression
    = l: literal() { Expression::Literal(l) } / "(" e: expression() ")" { e }

pub rule expression() -> Expression
    = o: add_operation() {o} / l: literal() { Expression::Literal(l) }

}
}
