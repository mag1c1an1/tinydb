use std::sync::Arc;
use crate::catalog::database::DatabaseCatalog;

mod column;
mod database;
mod schema;
mod table;

pub type DatabaseId = u32;
pub type SchemaId = u32;
pub type TableId = u32;
pub type ColumnId = u32;
pub type CatalogRef = Arc<DatabaseCatalog>;

pub const DEFAULT_SCHEMA_NAME: &str = "postgres";

pub struct TableRefId {
    pub schema_id: SchemaId,
    pub table_id: TableId,
}

pub struct ColumnRefId {
    pub schema_id: SchemaId,
    pub table_id: TableId,
    pub column_id: ColumnId,
}

#[derive(thiserror::Error, Debug)]
pub enum CatalogError {
    #[error("{0} not found: {1}")]
    NotFound(&'static str, String),
    #[error("duplicated {0}: {1}")]
    Duplicated(&'static str, String),
}
