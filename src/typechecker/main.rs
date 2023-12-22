use std::collections::HashMap;

use crate::types::ast::*;

pub fn entry(asts: &[FuncDeclar]) -> Result<(), String> {
    let mut fn_signs = HashMap::new();
    for f in asts {
        if !fn_signs.contains_key(&f.name) {
            fn_signs.insert(f.name.clone(), f.input_types.clone());
        } else {
            return Err(format!("Can't declare function twice: {:?}", f.location));
        }
    }
    for f in asts {
        for _ in &f.define.contents {}
    }
    Ok(())
}
