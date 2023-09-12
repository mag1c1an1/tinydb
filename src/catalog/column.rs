use crate::sql::types::DataType;

struct ColumnDesc{
  data_type: DataTypeRef,
  is_primary : bool,
}
impl ColumnDesc {
   pub(crate) fn new(data_type:impl DataType,is_primary:bool) {
      Self {
        data_type
      } 
   } 
}