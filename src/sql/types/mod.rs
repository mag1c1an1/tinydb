use std::str::FromStr;

#[repr(u8)]
#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub(crate) enum DataType {
    Int32,
    Bool,
    Float64,
    Char,
}

impl FromStr for DataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "int" | "int64" | "signed" | "integer" | "intergral" | "int32" => Ok(Self::Int32),
            "bool" | "boolean" | "logical" => Ok(Self::Bool),
            "double" | "float8" => Ok(Self::Float64),
            "char" | "bpchar" => Ok(Self::Char),
            _ => todo!("parse datatype"),
        }
    }
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::Int32 => "INTEGER",
            DataType::Bool => "BOOLEAN",
            DataType::Float64 => "DOUBLE",
            DataType::Char => "CHAR",
        }
        .into()
    }
}

impl DataType {
    pub fn data_len(&self) -> usize {
        match self {
            DataType::Int32 => 4,
            DataType::Bool => 1,
            DataType::Float64 => 8,
            DataType::Char => 1,
        }
    }
}

pub(crate) type DatabaseId = u32;
pub(crate) type SchemaId = u32;
pub(crate) type TableId = u32;
pub(crate) type ColumnId = u32;
