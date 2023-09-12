use thiserror::Error;
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] Box<std::io::Error>),
    #[error("Value error: {0}")]
    Value(String),
}

pub type StorageResult<T> = std::result::Result<T, StorageError>;
