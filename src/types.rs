pub use sqlparser::ast::DataType as DataTypeKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataType {
    kind: DataTypeKind,
    nullable: bool,
}

impl DataType {
    pub const fn new(kind: DataTypeKind, nullable: bool) -> Self {
        Self {
            kind,
            nullable,
        }
    }
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
    pub fn kind(&self) -> DataTypeKind {
        self.kind.clone()
    }
}

pub trait DataTypeExt {
    fn nullable(self) -> DataType;
    fn not_null(self) -> DataType;
}

impl DataTypeExt for DataTypeKind {
    fn nullable(self) -> DataType {
        DataType::new(self, true)
    }

    fn not_null(self) -> DataType {
        DataType::new(self, false)
    }
}