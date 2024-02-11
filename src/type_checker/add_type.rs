use std::collections::HashMap;

use super::ast_types as TIR;
use crate::ast_types as AST;

pub fn translate(ast: Vec<AST::Component>) -> Result<Vec<TIR::Component>, String> {
    let mut r = Vec::new();
    let mut ctx = Context {
        fns: HashMap::new(),
        vars: Vec::new(),
        type_aliases: HashMap::new(),
    };
    for a in &ast {
        match a {
            AST::Component::FnDeclar(f) => {
                ctx.fns.insert(
                    f.name.clone(),
                    AST::FnSignature {
                        name: f.name.clone(),
                        args: f.args.clone(),
                        return_type: f.return_type.clone(),
                    },
                );
            }
            AST::Component::FnSignature(e) => {
                ctx.fns.insert(e.name.clone(), e.clone());
            }
            AST::Component::TypeDeclar(t) => {
                ctx.type_aliases
                    .insert(t.name.clone(), (t.data_type.clone(), t.is_alias));
            }
            _ => (),
        }
    }
    for a in ast {
        r.push(match a {
            AST::Component::FnDeclar(f) => TIR::Component::FnDeclar(fn_declar(&mut ctx, f)?),
            AST::Component::RawJS(c) => TIR::Component::RawJS(c),
            AST::Component::FnSignature(e) => TIR::Component::FnSignature(TIR::FnSignature {
                name: e.name,
                args: e.args,
                return_type: e.return_type,
            }),
            AST::Component::TypeDeclar(t) => TIR::Component::TypeDeclar(t),
        })
    }
    Ok(r)
}

fn fn_declar(ctx: &mut Context, node: AST::FnDeclar) -> Result<TIR::FnDeclar, String> {
    ctx.vars.push({
        let mut hm = HashMap::new();
        for (k, v) in node.args.clone() {
            hm.insert(k, v);
        }
        hm
    });
    let block = block(ctx, node.block)?;
    ctx.vars.pop();
    Ok(TIR::FnDeclar {
        name: node.name,
        args: node.args,
        return_type: node.return_type,
        block,
    })
}

fn block(ctx: &mut Context, node: Vec<AST::Statement>) -> Result<Vec<TIR::Statement>, String> {
    let mut r = Vec::new();
    ctx.vars.push(HashMap::new());
    for s in node {
        r.push(match s {
            AST::Statement::While(w) => TIR::Statement::While(TIR::While {
                cond: expression(ctx, w.cond)?,
                block: block(ctx, w.block)?,
            }),
            AST::Statement::Block(b) => TIR::Statement::Block(block(ctx, b)?),
            AST::Statement::If(i) => TIR::Statement::If(TIR::If {
                then_cond: expression(ctx, i.then_cond)?,
                then_block: block(ctx, i.then_block)?,
                else_block: if let Some(e) = i.else_block {
                    Some(block(ctx, e)?)
                } else {
                    None
                },
            }),
            AST::Statement::VarUpdate(v) => {
                let w = expression(ctx, v.val)?;
                if let Some(t) = ctx.get_var(&v.name) {
                    if t != &w.data_type.normalize(ctx).0 {
                        return Err(format!("type unmatched: {}", v.name));
                    }
                    TIR::Statement::VarUpdate(TIR::VarUpdate {
                        name: v.name,
                        val: w,
                    })
                } else {
                    return Err(format!("variable {} doesn't exist", v.name));
                }
            }
            AST::Statement::VarDeclar(v) => {
                let l = ctx.vars.len() - 1;
                let t = v.data_type.normalize(ctx).0;
                ctx.vars[l].insert(v.name.clone(), t.clone());
                let w = expression(ctx, v.val)?;
                if t != w.data_type.normalize(ctx).0 {
                    return Err(format!("type unmatched: {}", v.name));
                }
                TIR::Statement::VarDeclar(TIR::VarDeclar {
                    name: v.name,
                    val: w,
                    data_type: t,
                })
            }
            AST::Statement::Expression(e) => TIR::Statement::Expression(expression(ctx, e)?),
        });
    }
    ctx.vars.pop();
    Ok(r)
}

fn expression(ctx: &mut Context, node: AST::Expression) -> Result<TIR::TypedExpression, String> {
    Ok(match node {
        AST::Expression::Literal(l) => TIR::TypedExpression {
            val: TIR::Expression::Literal(TIR::Literal {
                kind: l.kind.clone(),
                val: l.val,
            }),
            data_type: AST::DataType::Name(l.kind),
        },
        AST::Expression::Variable(v) => TIR::TypedExpression {
            val: TIR::Expression::Variable(TIR::Variable {
                name: v.name.clone(),
            }),
            data_type: ctx.get_var(&v.name).unwrap().normalize(ctx).0,
        },
        AST::Expression::Call(c) => {
            if let Some(fn_s) = ctx.clone_fn(&c.name) {
                TIR::TypedExpression {
                    val: TIR::Expression::Call(TIR::Call {
                        name: c.name.clone(),
                        args: {
                            let mut v = Vec::new();
                            if c.args.len() != ctx.fns.get(&c.name).unwrap().args.len() {
                                return Err(format!("unmatched args: {}", c.name));
                            }
                            for (i, arg) in c.args.into_iter().enumerate() {
                                let e = expression(ctx, arg)?;
                                if fn_s.args[i].1.normalize(ctx) != e.data_type.normalize(ctx) {
                                    return Err(format!(
                                        "unmatched arg: {:?} {:?}",
                                        fn_s.args[i].1, e.data_type
                                    ));
                                }
                                v.push(e)
                            }
                            v
                        },
                    }),
                    data_type: fn_s.return_type,
                }
            } else {
                return Err(format!("unknown function: {}", &c.name));
            }
        }
        AST::Expression::Object((n, b)) => {
            let mut m = HashMap::new();
            let mut t = HashMap::new();
            for (k, v) in b {
                let v = expression(ctx, v)?;
                m.insert(k.clone(), v.clone());
                t.insert(k, v.data_type.normalize(ctx).0);
            }
            TIR::TypedExpression {
                val: TIR::Expression::Object((n, m)),
                data_type: AST::DataType::Object(t),
            }
        }
        AST::Expression::Prop((e, a)) => {
            let te = expression(ctx, *e)?;
            let t = if let (AST::DataType::Object(s), _) = te.data_type.normalize(ctx) {
                if let Some(t) = s.get(&a) {
                    t.clone()
                } else {
                    return Err(format!("unknown property: {}", a));
                }
            } else {
                return Err(format!("primitive type doesn't have properties: {}", a));
            };
            TIR::TypedExpression {
                val: TIR::Expression::Prop((Box::new(te), a)),
                data_type: t.clone(),
            }
        }
        AST::Expression::Operation(o) => {
            let (left, right) = (expression(ctx, o.left)?, expression(ctx, o.right)?);
            let (left_d, right_d) = (left.data_type.clone(), right.data_type.clone());
            TIR::TypedExpression {
                val: TIR::Expression::Operation(Box::new(TIR::Operation {
                    kind: o.kind.clone(),
                    left,
                    right,
                })),
                data_type: {
                    if let (AST::DataType::Name(l), AST::DataType::Name(r)) = (left_d, right_d) {
                        match (&o.kind[..], &l[..], &r[..]) {
                            ("add", "int", "int") => AST::DataType::Name("int".to_string()),
                            ("sub", "int", "int") => AST::DataType::Name("int".to_string()),
                            ("mul", "int", "int") => AST::DataType::Name("int".to_string()),
                            ("div_1", "int", "int") => AST::DataType::Name("int".to_string()),
                            ("div_2", "int", "int") => AST::DataType::Name("int".to_string()),
                            ("add", "float", "float") => AST::DataType::Name("float".to_string()),
                            ("sub", "float", "float") => AST::DataType::Name("float".to_string()),
                            ("mul", "float", "float") => AST::DataType::Name("float".to_string()),
                            ("div_1", "float", "float") => AST::DataType::Name("float".to_string()),
                            ("div_2", "float", "float") => AST::DataType::Name("float".to_string()),
                            ("neq", l, r) if l == r => AST::DataType::Name("bool".to_string()),
                            ("eq", l, r) if l == r => AST::DataType::Name("bool".to_string()),
                            _ => unimplemented!("{} {} {}", &o.kind[..], &l[..], &r[..]),
                        }
                    } else {
                        unimplemented!()
                    }
                },
            }
        }
        AST::Expression::Cast((e, t)) => TIR::TypedExpression {
            val: TIR::Expression::Cast((Box::new(expression(ctx, *e)?), t.clone())),
            data_type: t,
        },
        AST::Expression::Index((a, i)) => {
            let e = expression(ctx, *a)?;
            let t = if let AST::DataType::Array(t) = e.data_type.normalize(ctx).0 {
                *t
            } else {
                return Err(format!("expression is not array"));
            };
            TIR::TypedExpression {
                val: TIR::Expression::Index((Box::new(e), Box::new(expression(ctx, *i)?))),
                data_type: t,
            }
        }
        AST::Expression::Array(v) => {
            let mut t = None;
            let mut a = Vec::new();
            for e in v {
                let e = expression(ctx, e)?;
                if let Some(ref t) = t {
                    if t != &e.data_type.normalize(ctx).0 {
                        return Err(format!("array elements must have the same type"));
                    }
                } else {
                    t = Some(e.data_type.normalize(ctx).0);
                }
                a.push(e);
            }
            TIR::TypedExpression {
                val: TIR::Expression::Array(a),
                data_type: AST::DataType::Array(Box::new(t.unwrap())),
            }
        }
    })
}
pub struct Context {
    fns: HashMap<String, AST::FnSignature>,
    vars: Vec<HashMap<String, AST::DataType>>,
    type_aliases: HashMap<String, (AST::DataType, bool)>,
}

impl Context {
    fn clone_fn(&self, n: &str) -> Option<AST::FnSignature> {
        if let Some(f) = self.fns.get(n) {
            Some(f.clone())
        } else {
            None
        }
    }
    fn get_var(&self, n: &str) -> Option<&AST::DataType> {
        for v in self.vars.iter().rev() {
            if v.contains_key(n) {
                return Some(v.get(n).unwrap());
            }
        }
        return None;
    }
    pub fn get_type(&self, n: &str) -> Option<&(AST::DataType, bool)> {
        if let Some(t) = self.type_aliases.get(n) {
            Some(t)
        } else {
            None
        }
    }
    //fn get_var_mut(&mut self, n: &str) -> Option<&mut DataType> {
    //    for v in self.vars.iter_mut().rev() {
    //        if v.contains_key(n) {
    //            return Some(v.get_mut(n).unwrap());
    //        }
    //    }
    //    return None;
    //}
}
