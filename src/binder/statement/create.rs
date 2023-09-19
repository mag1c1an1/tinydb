use super::*;
use crate::catalog::column::ColumnDesc;
use crate::catalog::SchemaId;

pub struct BoundCreateTable {
pub scheme_id:SchemaId,
    pub table_name: String,
    pub columns:Vec<(String,ColumnDesc)>,
}
impl Binder {
  pub fn bind_create_table(&mut self, stmt:&Statement) -> Result<BoundCreateTable,BindError>{
      todo!()
  }
}
