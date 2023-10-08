mod types;
mod lexer;
mod tests;
mod parser;
mod generator;
mod utils;

fn main() -> Result<(), String> {
    compile(r#"
fn main() -> void {
    let word: String = "Hello, world!";
    println(word);
}

"#.to_string())?;
    Ok(())
}

fn compile(code: String) -> Result<String,String> {
    let tokens = lexer::main(code);
    println!("{:#?}", tokens);
    let ast = parser::main::main(tokens)?;
    println!("{:#?}", ast);
    let code = generator::javascript::main::generate(ast);
    println!("{}", code);
    Ok(code)
}