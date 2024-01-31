mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::program(r#"
    let s = 0;
    s = s + 1;
"#).unwrap());
}