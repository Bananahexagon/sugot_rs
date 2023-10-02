mod types;
mod lexer;
mod tests;

fn main() {
    compile(r#"
fn main () -> void {
    println ( "Hello, world!" ) ;
}

"#.to_string());
}

fn compile(code: String) -> String {
    let tokens = lexer::main(code);
    println!("{:?}", tokens);
    unimplemented!()
}