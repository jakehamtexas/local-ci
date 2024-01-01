#[derive(Debug)]
pub enum Error {
    Other(Option<String>),
    IoNotFound,
    IoOther(std::io::Error),
    Serialization(serde_json::Error),
}

impl Error {
    pub fn other(message: &str) -> Error {
        Error::Other(Some(message.to_owned()))
    }

    pub fn _other_unspecified() -> Error {
        Error::Other(None)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        if value.kind() == std::io::ErrorKind::NotFound {
            Error::IoNotFound
        } else {
            Error::IoOther(value)
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serialization(value)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
