use crate::ast_types::Literal;

peg::parser! {
    pub grammar parser() for str {
        pub rule integer() -> Literal
            = n: $(['0'..='9']+) {Literal { kind: "integer".to_string(), val: n.to_string() } }
        pub rule string() -> Literal
            = s: $("\"" (!"\"" [_])* "\"") {Literal { kind: "string".to_string(), val: s.to_string() } }
        pub rule bool() -> Literal
            = s: $("true" / "false") {Literal { kind: "bool".to_string(), val: s.to_string() } }
        pub rule literal() -> Literal = integer() / string() / bool()
    }
}
