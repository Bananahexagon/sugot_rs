mod types;
mod lexer;
mod tests;
mod parser;

fn main() {
    compile(r#"
fn main() -> void {
    println("Hello, world!");
}

"#.to_string());
}

fn compile(code: String) -> String {
    let tokens = lexer::main(code);
    println!("{:?}", tokens);
    let _ast = parser::main::main(tokens);
    unimplemented!()
}