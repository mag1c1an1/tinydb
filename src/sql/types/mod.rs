use crate::storage::error::{StorageError, StorageResult};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Boolean => "BOOLEAN",
            Self::Integer => "INTEGER",
            Self::Float => "FLOAT",
            Self::String => "STRING",
            _ => "NOT SUPPORT",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
impl Value {
    pub fn datatype(&self) -> Option<DataType> {
        match self {
            Self::Null => None,
            Self::Boolean(_) => Some(DataType::Boolean),
            Self::Float(_) => Some(DataType::Float),
            Self::Integer(_) => Some(DataType::Integer),
            Self::String(_) => Some(DataType::String),
            _ => None,
        }
    }
    pub fn boolean(self) -> StorageResult<bool> {
        match self {
            Self::Boolean(b) => Ok(b),
            v => Err(StorageError::Value(format!("Not a bool: {:?}", v))),
        }
    }
    pub fn integer(self) -> StorageResult<i64> {
        match self {
            Self::Integer(b) => Ok(b),
            v => Err(StorageError::Value(format!("Not a integer: {:?}", v))),
        }
    }
    pub fn float(self) -> StorageResult<f64> {
        match self {
            Self::Float(f) => Ok(f),
            v => Err(StorageError::Value(format!("Not a float: {:?}", v))),
        }
    }
    pub fn string(self) -> StorageResult<String> {
        match self {
            Self::String(s) => Ok(s),
            v => Err(StorageError::Value(format!("Not a string: {:?}", v))),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Null => "NULL".to_string(),
                Self::Boolean(b) if *b => "TRUE".to_string(),
                Self::Boolean(_) => "FALSE".to_string(),
                Self::Integer(i) => i.to_string(),
                Self::Float(f) => f.to_string(),
                Self::String(s) => s.clone(),
            }
            .as_ref(),
        )
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(std::cmp::Ordering::Equal),
            (Value::Null, _) => Some(std::cmp::Ordering::Less),
            (_, Value::Null) => Some(std::cmp::Ordering::Greater),
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Integer(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Integer(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Integer(a), Value::Integer(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (_, _) => None,
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl std::cmp::Eq for Value {}

impl<'a> From<Value> for Cow<'a, Value> {
    fn from(value: Value) -> Self {
        Cow::Owned(value)
    }
}
impl<'a> From<&'a Value> for Cow<'a, Value> {
    fn from(value: &'a Value) -> Self {
        Cow::Borrowed(value)
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datatype().hash(state);
        match self {
            Value::Null => self.hash(state),
            Value::Boolean(v) => v.hash(state),
            Value::Integer(v) => v.hash(state),
            Value::Float(v) => v.to_be_bytes().hash(state),
            Value::String(v) => v.hash(state),
        }
    }
}

pub type Row = Vec<Value>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: Option<String>,
}

pub type Columns = Vec<Column>;
