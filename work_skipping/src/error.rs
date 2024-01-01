use crate::cache;
use crate::command;

#[derive(Debug)]
pub enum Error {
    Cache(cache::Error),
    Command(command::Error),
}

impl From<command::Error> for Error {
    fn from(value: command::Error) -> Self {
        Error::Command(value)
    }
}

impl From<cache::Error> for Error {
    fn from(value: cache::Error) -> Self {
        Error::Cache(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
