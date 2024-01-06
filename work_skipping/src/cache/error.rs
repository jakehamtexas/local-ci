#[derive(Debug)]
pub enum ReadError<T> {
    Other(T),
    Deserialization(serde_json::Error),
}

impl<T> From<serde_json::Error> for ReadError<T> {
    fn from(value: serde_json::Error) -> Self {
        ReadError::Deserialization(value)
    }
}

#[derive(Debug)]
pub enum WriteError<T> {
    Other(T),
    Serialization(serde_json::Error),
}

impl<T> From<serde_json::Error> for WriteError<T> {
    fn from(value: serde_json::Error) -> Self {
        WriteError::Serialization(value)
    }
}
