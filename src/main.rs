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
import { println } from std.io

fn main() -> unit {
    var i: i32 = 0;
    while i < 100 {
        i = i + 1;
        if i % 15 == 0 {
            println("FizzBuzz");
        } else if i % 5 == 0 {
            println("Buzz");
        } else if i % 3 == 0 {
            println("Fizz");
        } else {
            println(i);
        };
    };
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
