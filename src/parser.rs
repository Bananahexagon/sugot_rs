use crate::ast_types::*;

peg::parser! {
pub grammar parser() for str {

rule _ =  [' ' | '\t' | '\r' | '\n']*

rule identifier() -> String
    = s: $(!reserved() ['a'..='z' | 'A'..='Z' | '_']['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']*) { s.to_string() }

rule reserved() = ("true" / "false" / "let" / "if" / "else") !['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']

rule integer() -> Literal
    = n: $(['0'] / (['1'..='9']['0'..='9']*)) {Literal { kind: "integer".to_string(), val: n.to_string() } }

rule float() -> Literal
= n: $(['0'] / ['1'..='9']['0'..='9']*"."['0'..='9']) {Literal { kind: "float".to_string(), val: n.to_string() } }

rule string() -> Literal
    = s: $("\"" (!"\"" [_])* "\"") {Literal { kind: "string".to_string(), val: s.to_string() } }

rule bool() -> Literal
    = s: $("true" / "false") {Literal { kind: "bool".to_string(), val: s.to_string() } }

rule literal() -> Literal = integer() / string() / bool()

rule eq_operator() -> String = s: $("==" / "!=") { match s { "==" => "eq", "!=" => "neq",_=>unreachable!() }.to_string() }

rule eq_operation() -> Expression
    = l: add_operation() _ op: eq_operator() _ r: add_operation() { Operation { kind: op ,left:l, right:r }.into_expression() }
    / add_operation()

rule add_operator() -> String = s: $("+" / "-") { match s { "+" => "add", "-" => "sub",_=>unreachable!() }.to_string() }

rule add_operation() -> Expression
    = l: mul_operation() _ op: add_operator() _ r: add_operation() { Operation { kind: op ,left:l, right:r }.into_expression() }
    / mul_operation()


rule mul_operator() -> String = s: $("*" / "/" / "%") { match s { "*" => "mul", "/" => "div_1", "%" => "div_2",_=>unreachable!() }.to_string() }

rule mul_operation() -> Expression
    = l: expression_atom() _ op: mul_operator() _ r: mul_operation() { Operation { kind: op ,left:l, right:r }.into_expression() }
    / expression_atom()

rule expression_atom() -> Expression
    = l: literal() { Expression::Literal(l) } / i: identifier() { Expression::Variable(Variable { name: i }) } / "(" e: expression() ")" { e }

rule call() -> Expression
    = i: identifier() "(" a: expression() ** (_ "," _) ")" { Call { name: i, args: a }.into_expression()}

rule expression() -> Expression
    = c: call() { c }
    / o: eq_operation() { o }
    / l: literal() { Expression::Literal(l) }
    / i: identifier() { Expression::Variable(Variable { name: i }) }

rule var_declar() -> Statement
    = "let" _ n: identifier() _ "=" _ e: expression() ";" { Statement::VarDeclar(VarDeclar{ name: n, val: e }) }

rule var_update() -> Statement
    = n: identifier() _ "=" _ e: expression() ";" { Statement::VarUpdate(VarUpdate{ name: n, val: e }) }

rule block() -> Vec<Statement>
    = "{" _ s: statement() ** _ _ "}" { s }

rule statement() -> Statement
    = e: expression() ";" { Statement::Expression(e) }
    / d: var_declar() { d }
    / u: var_update() { u }
    / b: block() { Statement::Block(b) }
    / w: while() { w }
    / i: if() { i }

rule if() -> Statement
    = "if " _ c: expression() _ b: block() _ "else" _ e: block() { Statement::If( If { then_cond: c, then_block: b, else_block: Some(e) } ) }
    / "if " _ c: expression() _ b: block() _ "else" _ e: if() { Statement::If( If { then_cond: c, then_block: b, else_block: Some(vec![e]) } ) }
    / "if " _ c: expression() _ b: block() { Statement::If( If { then_cond: c, then_block: b, else_block: None } ) }

rule while() -> Statement
    = "while " _ c: expression() _ b: block() { Statement::While( While { cond: c, block: b } ) }

pub rule program() -> Vec<Statement>
    = _ p: statement() ** _ _  { p }
}
}
