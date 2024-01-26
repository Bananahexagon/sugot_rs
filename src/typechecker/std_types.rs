use crate::types::{
    ast::{DataType, FuncArgs},
    general::Location,
};

pub fn types(s: &str) -> Option<(Vec<FuncArgs>, String)> {
    let location = Location {
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 0,
    };
    let b = match s {
        "io.println" => Some((vec![("arg_0", "string")], "void".to_string())),
        _ => None,
    };
    if let None = b {
        return None;
    };
    let b = b.unwrap();
    let mut rv = Vec::new();
    for c in b.0 {
        rv.push(FuncArgs {
            location: location.clone(),
            name: c.0.to_string(),
            data_type: DataType {
                location: location.clone(),
                val: c.1.to_string(),
            },
        })
    }
    return Some((rv, b.1));
}
