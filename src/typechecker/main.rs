use std::collections::{HashMap, HashSet};

use crate::types::ast::*;

pub fn entry(asts: &[FuncDeclar]) -> Result<(), String> {
    let applicable = {
        let mut t: HashSet<(&'static str, &'static str)> = HashSet::new();
        t.insert(("i32", "int"));
        t
    };
    let mut fn_signs = HashMap::new();
    for f in asts {
        if !fn_signs.contains_key(&f.name) {
            fn_signs.insert(f.name.clone(), f.input_types.clone());
        } else {
            return Err(format!("Can't declare function twice: {:?}", f.location));
        }
    }
    let mut var_signs = HashMap::new();
    for f in asts {
        dfs(&f.define,&mut fn_signs,&mut var_signs)
    }

    Ok(())
}

fn dfs(block: &Block,fn_signs: &mut HashMap<String, Vec<FuncArgs>>,var_signs: &mut HashMap<String, Vec<DataType>>) {
    for _ in &block.contents {}
}
