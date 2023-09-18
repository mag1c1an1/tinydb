use sqlparser::parser::ParserError;
use crate::executor::{execute, ExecuteError};
use crate::parser::parse;

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&self, sql: &str) -> Result<Vec<String>, Error> {
        let stmts = parse(sql)?;
        let mut outputs = vec![];
        for stmt in stmts {
            debug!("execute: {:#?}",stmt);
            let output = execute(&stmt);
            outputs.extend(output);
        }
        Ok(outputs)
    }
}


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(#[from] ParserError),
    #[error("execute error: {0}")]
    Execute(#[from] ExecuteError),
}
