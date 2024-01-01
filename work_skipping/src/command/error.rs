#[derive(Debug)]
pub enum Error {
    CreationFailed,
    Io(std::io::Error),
    Cache(crate::cache::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<crate::cache::Error> for Error {
    fn from(value: crate::cache::Error) -> Self {
        Error::Cache(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
