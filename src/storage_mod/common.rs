use std::{error, result};
use thiserror::Error;
use raft::{Error as RaftError, StorageError};

#[derive(Debug, Error)]
pub enum Error{
    #[error("Storage Engine error {0}")]
    Engine(String),
    #[error("unavailable entries")]
    EntriesUnavailable,
    #[error("compacted entries")]
    EntriesCompacted,
}

pub type Result<T> = result::Result<T, Error>;

impl From<String> for Error{
    fn from(err: String) -> Self{
        Error::Engine(err)
    }
}

impl From<Error> for RaftError{
    fn from(e: Error) -> RaftError{
        match e{
            Error::EntriesUnavailable => RaftError::Store(StorageError::Unavailable),
            Error::EntriesCompacted => RaftError::Store(StorageError::Compacted),
            e => {
                let boxed = Box::new(e) as Box<dyn std::error::Error + Sync + Send>;
                raft::Error::Store(StorageError::Other(boxed))
            }
        }
    }
}

