use std::fs;
use std::io::Write;

mod ast_types;
mod generator;
mod parser;
mod json;

fn main() {
    let option = fs::read_to_string("./examples/project.json").unwrap_or_else(|op| {
        eprintln!("Error: {:?}", op);
        String::new()
    });
    let config = json::parser::json(&option).unwrap();
    let c_dir = config.obj().unwrap().get("entry").unwrap().str().unwrap();
    let code = fs::read_to_string(c_dir).unwrap_or_else(|op| {
        eprintln!("Error: {:?}", op);
        String::new()
    });
    let parsed = parser::parser::program(&code).unwrap();
    let result = generator::entry::generate(parsed);
    let o_dir = config.obj().unwrap().get("outdir").unwrap().str().unwrap();
    if let Ok(c) = result {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(o_dir)
            .unwrap();
        file.write_all(c.as_bytes()).unwrap();
    } else if let Err(e) = result {
        eprintln!("{}", e);
        return;
    };
}
