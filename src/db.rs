use std::sync::Arc;
use sqlparser::parser::ParserError;
use crate::binder::Binder;
use crate::catalog::CatalogRef;
use crate::catalog::database::DatabaseCatalog;
use crate::executor::{execute, ExecuteError};
use crate::parser::parse;

pub struct Database {
    catalog: CatalogRef
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}


impl Database {
    pub fn new() -> Self {
        let catalog = Arc::new(DatabaseCatalog::new());
        Database { catalog }
    }
    pub fn run(&self, sql: &str) -> Result<Vec<String>, Error> {
        let stmts = parse(sql)?;
        let mut binder = Binder::new(self.catalog.clone());
        let executor = Executor::new();
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
