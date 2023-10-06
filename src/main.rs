mod types;
mod lexer;
mod tests;
mod parser;
mod generator;

fn main() {
    compile(r#"
fun main() -> void {
    println("Hello, world!");
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