mod parser;
mod ast_types;

fn main() {
    dbg!(parser::parser::program(r#"
let s = "Hello, world!";
println("{}", s);
"#).unwrap());
}