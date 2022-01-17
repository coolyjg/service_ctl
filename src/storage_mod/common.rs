use std::{error, result};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error{
    #[error("Storage Engine error {0}")]
    Engine(String),
}

pub type Result<T> = result::Result<T, Error>;

impl From<String> for Error{
    fn from(err: String) -> Self{
        Error::Engine(err)
    }
}

