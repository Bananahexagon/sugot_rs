use std::collections::HashMap;

use crate::ast_types::*;

peg::parser! {
pub grammar parser() for str {

rule _ =  [' ' | '\t' | '\r' | '\n']*

rule identifier() -> String
    = s: $(!reserved() ['a'..='z' | 'A'..='Z' | '_']['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']*) { s.to_string() }

rule reserved() = ("true" / "false" / "let" / "if" / "else") !['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']

rule integer() -> Literal
    = n: $(['0'] / (['1'..='9']['0'..='9']*)) {Literal { kind: "int".to_string(), val: n.to_string() } }

rule float() -> Literal
= n: $(['0'] / ['1'..='9']['0'..='9']*"."['0'..='9']) {Literal { kind: "float".to_string(), val: n.to_string() } }

rule string() -> Literal
    = s: $("\"" (!"\"" [_])* "\"") {Literal { kind: "string".to_string(), val: s.to_string() } }

rule bool() -> Literal
    = s: $("true" / "false") {Literal { kind: "bool".to_string(), val: s.to_string() } }

rule object() -> (String, HashMap<String, Expression>)
    = n: identifier() _ "{" m: obj_member() ** (_ "," _) _ "}" {
        let mut h = HashMap::new();
        for (n, v) in m {
            h.insert(n, v);
        }
        (n, h)
    }

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
    = l: literal() { Expression::Literal(l) } 
    / i: identifier() { Expression::Variable(Variable { name: i }) } 
    / "(" e: expression() ")" { e }

rule call() -> Expression
    = i: identifier() "(" a: expression() ** (_ "," _) ")" { Call { name: i, args: a }.into_expression()}
    / o: expression_atom() "." i: identifier() "(" a: expression() ** (_ "," _) ")" { 
        let mut b = Vec::new();
        b.push(o);
        for a in a {
            b.push(a)
        }
        Call { name: i, args: b }.into_expression()
    }

rule expression() -> Expression
    = c: call() { c }
    / e: expression_atom() " " _ "as " _  t: data_type() { Expression::Cast((Box::new(e), t)) }
    / o: eq_operation() { o }
    / o: object() { Expression::Object(o) }
    / o: expression_atom() "." p: identifier() { Expression::Prop((Box::new(o), p)) }
    / a: expression_atom() { a }

rule data_type() -> DataType
    = n: identifier() { DataType::Name(n) }
    / n: identifier() _ "{" m: type_member() ** (_ "," _) _ "}" {
        let mut h = HashMap::new();
        for (n, v) in m {
            h.insert(n, v);
        }
        DataType::Object(h)
    }

rule var_declar() -> Statement
    = "let" _ n: identifier() _ ":" _ t: data_type() _ "=" _ e: expression() ";" { Statement::VarDeclar(VarDeclar{ name: n, data_type: t, val: e }) }

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

rule fn_declar() -> FnDeclar
    = "fn " _ n: identifier() _ "(" _ a: fn_type() ** (_ "," _) _ ")" _ ":" _ r: data_type() _ b: block()
    { FnDeclar { name: n, args: a, return_type: r, block: b } }

rule fn_type() -> (String, DataType)
    = n: identifier() _ ":" _ t: data_type() {( n, t )}

rule fn_extern() -> FnSignature
    = "extern fn " _ n: identifier() _ "(" _ a: fn_type() ** (_ "," _) _ ")" _ ":" _ r: data_type() { FnSignature { name: n, args: a, return_type: r } }

rule obj_member() -> (String, Expression)
    = n: identifier() _ ":" _ t: expression() {( n, t )}

rule type_member() -> (String, DataType)
    = n: identifier() _ ":" _ t: data_type() {( n, t )}

rule component() -> Component
    = f: fn_declar() { Component::FnDeclar(f) }
    / c: "#raw_js(" s: $((!")#" [_])*) ")#" { Component::RawJS(s.to_string()) }
    / e: fn_extern() { Component::FnSignature(e) }

pub rule program() -> Vec<Component>
    = _ c: component() ** _ _  { c }
}
}
