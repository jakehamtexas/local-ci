use crate::cache;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Cache(cache::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<cache::Error> for Error {
    fn from(value: cache::Error) -> Self {
        Error::Cache(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
