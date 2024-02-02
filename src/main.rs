mod ast_types;
mod generator;
mod parser;

fn main() {
    let parsed = parser::parser::program(
        r#"
let i: int = 0;
while i != 100 {
    if i % 15 == 0 {
        println("FizzBuzz");
    } else if i % 3 == 0 {
        println("Fizz");
    } else if i % 3 == 0 {
        println("Buzz");
    } else {
        println(i);
    }
    i = i + 1;
}
"#,
    ).unwrap();
    println!("{}", generator::entry::generate(parsed).unwrap());
}
