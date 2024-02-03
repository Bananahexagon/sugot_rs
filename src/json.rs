use std::collections::HashMap;

use peg;

peg::parser!(
pub grammar parser() for str {

pub rule json() -> Value
    = _ o: object() _ { Value::Obj(o) }
    / _ a: array() _ { Value::Arr(a) }

rule _ = [' ' | '\t' | '\r' | '\n']*
rule value_separator() = _ "," _

rule value() -> Value
    = "false" { Value::Bool(false) }
    / "true" { Value::Bool(true) }
    / o: object() { Value::Obj(o) }
    / v: array() { Value::Arr(v) }
    / i: int() { Value::Int(i) }
    / s: string() {Value::Str(s)}

rule object() -> HashMap<String, Value>
    = "{" _ m: member() ** value_separator() _ "}" {
        let mut r = HashMap::new();
        for (k, v) in m {
            r.insert(k, v);
        }
        r
    }

rule member() -> (String, Value)
    = s: string() _ ":" _ v: value() { (s, v) }

rule array() -> Vec<Value>
    = "[" _ v: value() ** value_separator() _ "]" { v }

rule int() -> u32
    = i: $(['0'] / ['1'..='9']['0'..='9']*) { i.parse::<u32>().unwrap() }

// note: escaped chars not handled
rule string() -> String
    = "\"" s: $((!"\"" [_])*) "\"" { s.to_string() }
});

pub enum Value {
    Int(u32),
    Str(String),
    Bool(bool),
    Obj(HashMap<String, Value>),
    Arr(Vec<Value>),
}

impl Value {
    pub fn int(&self) -> Option<&u32> {
        if let Value::Int(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn str(&self) -> Option<&String> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn bool(&self) -> Option<&bool> {
        if let Value::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }
    pub fn obj(&self) -> Option<&HashMap<String, Value>> {
        if let Value::Obj(o) = self {
            Some(o)
        } else {
            None
        }
    }
    pub fn arr(&self) -> Option<&Vec<Value>> {
        if let Value::Arr(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
