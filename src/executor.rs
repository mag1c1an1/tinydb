use std::fmt::Write;
use crate::catalog::CatalogRef;
use crate::parser::{Expr, SelectItem, SetExpr, Statement, Value};


pub struct Executor {
    catalog: CatalogRef,
}

impl Executor {
    pub fn new(catalog: CatalogRef) -> Self {
        Executor { catalog }
    }
}

// TODO
pub fn execute(stmt: &Statement) -> Result<String, ExecuteError> {
    match stmt {
        Statement::Query(query) => match &*query.body {
            SetExpr::Select(select) => {
                let mut output = String::new();
                for item in &select.projection {
                    write!(output, " ").unwrap();
                    match item {
                        SelectItem::UnnamedExpr(Expr::Value(v)) => match v {
                            Value::SingleQuotedString(s) => write!(output, "{}", s).unwrap(),
                            Value::Number(s, _) => write!(output, "{}", s).unwrap(),
                            _ => todo!("unsupported statement: {:#?}", stmt),
                        }
                        _ => todo!("unsupported statement: {:#?}", stmt),
                    }
                }
                return Ok(output.trim().to_string());
            }
            _ => todo!("unsupported statement: {:#?}", stmt),
        }
        _ => todo!("unsupported statement: {:#?}", stmt),
    }
}


#[derive(thiserror::Error, Debug)]
pub enum ExecuteError {}
