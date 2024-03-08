use std::fs;
use std::io::Write;

mod ast_types;
mod generator_cpp;
mod generator_js;
mod json;
mod parser;
mod type_checker;

fn main() {
    let option = fs::read_to_string("./sample/project.json").unwrap_or_else(|op| {
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
    let typed = type_checker::add_type::translate(parsed).unwrap();
    //if let Err(s) = type_checker::checker::main(typed.clone()) {
    //    eprintln!("{}", s);
    //    return;
    //}
    let o_dir = config.obj().unwrap().get("outdir").unwrap().str().unwrap();
    let result = match o_dir.split('.').last().unwrap() {
        "js" => generator_js::entry::generate(typed),
        "cpp" => generator_cpp::entry::generate(typed),
        _ => unimplemented!(),
    };
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
