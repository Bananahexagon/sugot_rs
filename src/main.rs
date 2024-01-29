mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::program(r#"
    s = s + 1;
    s += 1;
"#).unwrap());
}