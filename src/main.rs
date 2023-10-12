mod types;
mod lexer;
mod tests;
mod parser;
mod generator;
mod utils;
mod typechecker;

fn main() -> Result<(), String> {
    compile(r#"
fn main() -> unit {
    let a: int = 5;
    println(1 + 2);
    return unit;
}

"#.to_string())?;
    Ok(())
}

fn compile(code: String) -> Result<String,String> {
    let tokens = lexer::main(code);
    println!("{:#?}", tokens);
    let ast = parser::main::main(tokens)?;
    println!("{:#?}", ast);
    typechecker::main::entry(&ast)?;
    let code = generator::javascript::main::generate(ast);
    println!("{}", code);
    Ok(code)
}