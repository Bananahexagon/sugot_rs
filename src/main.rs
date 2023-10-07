mod types;
mod lexer;
mod tests;
mod parser;
mod generator;
mod utils;

fn main() {
    compile(r#"
fn main() -> void {
    println(succ(0));
}

"#.to_string());
}

fn compile(code: String) -> String {
    let tokens = lexer::main(code);
    println!("{:#?}", tokens);
    let ast = parser::main::main(tokens);
    println!("{:#?}", ast);
    let code = generator::javascript::main::generate(ast);
    println!("{}", code);
    code
}