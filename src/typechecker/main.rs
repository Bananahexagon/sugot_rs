use std::collections::HashMap;

use super::std_types::types;
use crate::types::ast::*;

pub fn entry(asts: &[Declars]) -> Result<(), String> {
    let mut fn_signs = HashMap::new();
    for f in asts {
        match f {
            Declars::Func(f) => {
                if !fn_signs.contains_key(&f.name) {
                    fn_signs.insert(f.name.clone(), f.input_types.clone());
                } else {
                    return Err(format!("Can't declare function twice: {:?}", f.location));
                }
            }
            Declars::Import(im) => {
                if im.path[0] == "std" {
                    let dirstr = {
                        let mut s = String::new();
                        for p in &im.path[1..] {
                            s.push_str(&format!("{}.", p));
                        }
                        s
                    };
                    for f in im.contents.iter() {
                        if let Some(t) = types(&format!("{}{}", dirstr, f)) {
                            fn_signs.insert(f.clone(), t.0);
                        } else {
                            return Err(format!("Function {} not found in std", f));
                        }
                    }
                }
            }
        }
    }
    let mut var_signs = vec![];
    for d in asts {
        if let Declars::Func(f) = d {
            dfs(&f.define, &mut fn_signs, &mut var_signs, 0, &f.return_type)?
        }
    }

    Ok(())
}

fn dfs(
    block: &Block,
    fn_signs: &mut HashMap<String, Vec<FuncArgs>>,
    var_signs: &mut Vec<HashMap<String, DataType>>,
    layer: u32,
    return_type: &DataType,
) -> Result<(), String> {
    var_signs.push(HashMap::new());
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
            Statement::Call(c) => {
                //TODO 2021-12-23 この辺やばい
                if let Some(args) = fn_signs.get(&c.func) {
                    if !args
                        .iter()
                        .all(|arg| is_match(&arg.data_type.val, "allowed"))
                    {
                        return Err(format!(""));
                    };
                } else {
                    return Err(format!("function {} didn't exist", c.func));
                }
            }
            Statement::Return(_r) => {
                //TODO 2023-12-23 この辺やばい
                if !is_match(&return_type.val, "allowed") {
                    return Err(format!("return type and expression type was different"));
                }
            }
            Statement::VarDeclar(v) => {
                //TODO 2023-12-23 この辺やばい
                if let Some(_init) = &v.init {
                    if !is_match(&v.data_type.val, "allowed") {
                        return Err(format!("initialize was failed"));
                    }
                }
                let l = var_signs.len();
                var_signs[l - 1].insert(v.name.clone(), v.data_type.clone());
            }
            Statement::VarSet(v) => {
                //TODO 2023-12-23 この辺もやばい
                if let Some(i) = var_exist(var_signs, &v.name) {
                    if !is_match(&var_signs[i].get(&v.name).unwrap().val, "allowed") {
                        return Err(format!("OTT"));
                    };
                } else {
                    return Err(format!("OTINTIN"));
                }
            }
        }
    }
    var_signs.pop();
    Ok(())
}

fn is_match(l: &str, r: &str) -> bool {
    l == r || l == "any" || r == "allowed"
}

fn var_exist(var_signs: &Vec<HashMap<String, DataType>>, name: &str) -> Option<usize> {
    for (i, m) in var_signs.iter().enumerate().rev() {
        if m.contains_key(name) {
            return Some(i);
        }
    }
    None
}
