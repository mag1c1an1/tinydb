use crate::catalog::ColumnId;
use crate::types::DataType;

#[derive(Clone)]
pub struct ColumnDesc {
    data_type: DataType,
    is_primary: bool,
}

impl ColumnDesc {
    pub const fn new(data_type: DataType, is_primary: bool) -> Self {
        Self {
            data_type,
            is_primary,
        }
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn is_nullable(&self) -> bool {
        self.data_type.is_nullable()
    }
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}

impl DataType {
    pub const fn to_column(self) -> ColumnDesc {
        ColumnDesc::new(self, false)
    }
    pub const fn to_column_primary_key(self) -> ColumnDesc {
        ColumnDesc::new(self, true)
    }
}

#[derive(Clone)]
pub struct ColumnCatalog {
    id: ColumnId,
    name: String,
    desc: ColumnDesc,
}

impl ColumnCatalog {
    pub(super) fn new(id: ColumnId, name: String, desc: ColumnDesc) -> Self {
        Self { id, name, desc }
    }

    pub fn id(&self) -> ColumnId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataType {
        self.desc.data_type.clone()
    }

    pub fn is_primary(&self) -> bool {
        self.desc.is_primary()
    }

    pub fn is_nullable(&self) -> bool {
        self.desc.is_nullable()
    }
}
