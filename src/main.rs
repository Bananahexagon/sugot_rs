mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::expression(r#"(0+123)*4+5*6*7+8"#).unwrap());
}