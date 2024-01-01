use super::fs;

#[derive(Debug)]
pub enum Error {
    Fs(fs::error::Error),
}

impl From<fs::error::Error> for Error {
    fn from(value: fs::error::Error) -> Self {
        Error::Fs(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
