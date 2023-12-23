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
    let mut var_signs = HashMap::new();
    for f in asts {
        dfs(&f.define, &mut fn_signs, &mut var_signs, 0, &f.return_type)?
    }

    Ok(())
}

fn dfs(
    block: &Block,
    fn_signs: &mut HashMap<String, Vec<FuncArgs>>,
    var_signs: &mut HashMap<String, Vec<DataType>>,
    layer: u32,
    return_type: &DataType,
) -> Result<(), String> {
    for s in &block.contents {
        match s {
            Statement::Block(b) => dfs(b, fn_signs, var_signs, layer + 1, return_type)?,
            Statement::If(i) => {
                dfs(
                    &i.then_contents,
                    fn_signs,
                    var_signs,
                    layer + 1,
                    return_type,
                )?;
                if let Some(c) = &i.else_contents {
                    dfs(&c, fn_signs, var_signs, layer + 1, return_type)?
                }
            }
            Statement::While(w) => dfs(&w.contents, fn_signs, var_signs, layer + 1, return_type)?,
            Statement::Call(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::VarDeclar(_) => todo!(),
            Statement::VarSet(_) => todo!(),
        }
    }
    Ok(())
}

fn _is_match(l: &str, r: &str) -> bool {
    l == r || l == "any" || r == "allowed"
}
