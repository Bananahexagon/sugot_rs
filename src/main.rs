mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::string(r#""Hello, world!""#).unwrap());
}