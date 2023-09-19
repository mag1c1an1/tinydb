use crate::binder::statement::create::BoundCreateTable;
use crate::binder::statement::select::BoundSelect;
use crate::catalog::CatalogRef;
use crate::parser::{Ident, ObjectName, Statement};

pub mod statement;

pub enum BoundStatement
{
    CreateTable(BoundCreateTable),
    Select(BoundSelect),
}

pub struct Binder {
    catalog: CatalogRef,
}

impl Binder {
    pub fn new(catalog: CatalogRef) -> Self {
        Binder {
            catalog
        }
    }

    pub fn bind(&mut self,stmt: &Statement) -> Result<BoundStatement,BindError>
    {
        use Statement::*;
        match stmt {
            CreateTable {..} => Ok(BoundStatement::CreateTable(self.bind_create_table(stmt)?)),
            _ => todo!()
        }
    }
}

fn split_name(name: &ObjectName) -> Result<(&str, &str), BindError> {
    todo!()
}


/// The error type of bind operations.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BindError {
    #[error("schema not found: {0}")]
    SchemaNotFound(String),
}
