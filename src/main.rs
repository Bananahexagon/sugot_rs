mod ast_types;
mod generator;
mod parser;

fn main() {
    let parsed = parser::parser::program(
        r#"
    let s = 0;
    s = s + 1;
"#,
    ).unwrap();
    println!("{}", generator::entry::generate(parsed).unwrap());
}
