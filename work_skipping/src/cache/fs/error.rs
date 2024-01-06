use common::canonicalized_path;

#[derive(Debug)]
pub enum Error {
    IoNotFound,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        if value.kind() == std::io::ErrorKind::NotFound {
            Error::IoNotFound
        } else {
            Error::Io(value)
        }
    }
}

impl From<canonicalized_path::Error> for Error {
    fn from(value: canonicalized_path::Error) -> Self {
        match value {
            canonicalized_path::Error::Read(e)
            | canonicalized_path::Error::Write(e)
            | canonicalized_path::Error::Creation(e) => e.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
