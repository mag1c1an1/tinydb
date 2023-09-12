use crate::sql::types::{DataType, ColumnId};

pub(crate) struct ColumnDesc {
    data_type: DataType,
    is_primary: bool,
    is_nullable: bool,
}
impl ColumnDesc {
    pub fn new(data_type: DataType, is_primary: bool, is_nullable: bool) -> Self {
        Self {
            data_type,
            is_primary,
            is_nullable,
        }
    }
    pub fn set_primary(&mut self, is_primary: bool) {
        self.is_primary = is_primary;
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }
    pub fn data_type(&self) -> DataType {
        self.data_type
    }
}
pub(crate) struct ColumnCatalog {
    id: ColumnId,
    name: String,
    desc: ColumnDesc,
}

impl ColumnCatalog {
    pub fn new(id: ColumnId, name: String, desc: ColumnDesc) -> Self {
        Self { id, name, desc }
    }

    pub fn id(&self) -> ColumnId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn datatype(&self) -> DataType {
        self.desc.data_type()
    }

    pub fn set_primary(&mut self, is_primary: bool) {
        self.desc.set_primary(is_primary);
    }

    pub fn is_primary(&self) -> bool {
        self.desc.is_primary()
    }

    pub fn is_nullable(&self) -> bool {
        self.desc.is_nullable()
    }
}
