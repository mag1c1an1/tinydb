use std::{any::Any, sync::Arc};

mod bool_type;
mod numeric_type;

pub enum DataTypeEnum {
    Int32,
    Bool,
    Float64,
    Char,
}
/// Inner data type
pub trait DataType {
    fn is_nullable(&self) -> bool;
    fn get_type(&self) -> DataTypeEnum;
    fn data_len(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

pub(crate) type DataTypeRef = Arc<dyn DataType>;
