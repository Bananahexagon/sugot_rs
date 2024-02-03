mod ast_types;
mod generator;
mod parser;

fn main() {
    let parsed = parser::parser::program(
        r#"
#raw_js(
    const $sugot_println = console.log;
)#

fn main(): void {
    let i: int = 0;
    while i != 100 { i.fizz_buzz(); i = i + 1; }
}

fn fizz_buzz(i: int): void {
    if i % 15 == 0 {
        println("FizzBuzz");
    } else if i % 3 == 0 {
        println("Fizz");
    } else if i % 3 == 0 {
        println("Buzz");
    } else {
        println(i);
    }
}
"#,
    ).unwrap();
    println!("{}", generator::entry::generate(parsed).unwrap());
}
