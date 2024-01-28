use crate::ast_types::*;

peg::parser! {
pub grammar parser() for str {

rule _ = " " / "\n" / "\t" / "\r"

rule integer() -> Literal
= n: $(['0'] / ['1'..='9']['0'..='9']*) {Literal { kind: "integer".to_string(), val: n.to_string() } }

rule string() -> Literal
= s: $("\"" (!"\"" [_])* "\"") {Literal { kind: "string".to_string(), val: s.to_string() } }

rule bool() -> Literal
= s: $("true" / "false") {Literal { kind: "bool".to_string(), val: s.to_string() } }

rule literal() -> Literal = integer() / string() / bool()

rule add_operator() -> String = s: $("+" / "-") { match s { "+" => "add", "-" => "sub",_=>unreachable!() }.to_string() }

rule add_operation() -> Operation
= l: expression_atom() _ op: add_operator() _ r: expression_atom() { Operation { kind: op ,left:l, right:r } }

rule expression_atom() -> Expression
= l: literal() { Expression::Literal(l) } / "(" e: expression() ")" { e }

pub rule expression() -> Expression
= o: add_operation() {Expression::Operation(Box::new(o))} / l: literal() { Expression::Literal(l) }

}
}
