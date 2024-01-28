mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::expression(r#"123 + 1"#).unwrap());
}