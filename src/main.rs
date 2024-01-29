mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::expression(r#"println("Hello, world!")"#).unwrap());
}