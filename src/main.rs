mod generator;
mod lexer;
mod parser;
mod tests;
mod typechecker;
mod types;
mod utils;

fn main() -> Result<(), String> {
    compile(
        r#"
fn main() -> unit {
    println(five());
}

fn five(arg: i32) -> i32 {
    return 5;
}
"#
        .to_string(),
    )?;
    Ok(())
}

fn compile(code: String) -> Result<String, String> {
    let tokens = lexer::main(code);
    println!("{:#?}", tokens);
    let ast = parser::main::main(tokens)?;
    println!("{:#?}", ast);
    typechecker::main::entry(&ast)?;
    let code = generator::javascript::main::generate(ast);
    println!("{}", code);
    Ok(code)
}
