use std::path::PathBuf;
use thiserror::Error;

pub type FileResult<T, E = FileError> = std::result::Result<T, E>;

#[derive(Debug, Error, Clone)]
#[error("FileError: {:?}", self)]
pub struct FileError<E: std::error::Error = std::io::Error> {
    #[source]
    source: E,
    path: PathBuf,
    message: Option<&'static str>,
}

impl PartialEq for FileError {
    fn eq(&self, other: &Self) -> bool {
        self.source.kind() == other.source.kind()
            && self.path == other.path
            && self.message == other.message
    }
}

impl<E: std::error::Error> FileErrorBuilder<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: std::error::Error> FileError<T> {
    pub fn new(source: T, path: PathBuf, message: Option<&'static str>) -> Self {
        FileError {
            source,
            path,
            message,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn source(&self) -> &T {
        &self.source
    }

    pub fn into_builder(self) -> FileErrorBuilder<T> {
        FileErrorBuilder {
            source: Some(self.source),
            path: Some(self.path),
            message: self.message,
        }
    }
}

impl<E: std::error::Error> From<E> for FileError<E> {
    fn from(value: E) -> Self {
        FileErrorBuilder::<E>::new().with_source(value).build()
    }
}

// TODO: Might not need this
#[derive(Debug, Clone)]
pub struct FileErrorBuilder<E: std::error::Error> {
    source: Option<E>,
    path: Option<PathBuf>,
    message: Option<&'static str>,
}

impl<E: std::error::Error> FileErrorBuilder<E> {
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_source(mut self, source: E) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_message(mut self, message: &'static str) -> Self {
        self.message = Some(message);
        self
    }

    pub fn build(self) -> FileError<E> {
        FileError {
            path: self.path.expect("Must have path"),
            source: self.source.expect("Must have a source"),
            message: self.message,
        }
    }
}

impl<E: std::error::Error> Default for FileErrorBuilder<E> {
    fn default() -> Self {
        FileErrorBuilder {
            path: None,
            source: None,
            message: None,
        }
    }
}

impl<E: std::error::Error> From<E> for FileErrorBuilder<E> {
    fn from(value: E) -> Self {
        FileErrorBuilder::default().with_source(value)
    }
}

pub type FileHandleResult<T, E = FileHandleError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum FileHandleError<E: std::error::Error = std::io::Error> {
    #[error("Failed to create path {0}")]
    Create(#[source] FileError<E>),
    #[error("Failed to read path {0}")]
    Read(#[source] FileError<E>),
    #[error("Failed to write path {0}")]
    Write(#[source] FileError<E>),
}

impl<E: std::error::Error> FileHandleError<E> {
    pub fn inner(&self) -> &FileError<E> {
        use FileHandleError as FHE;
        match self {
            FHE::Create(e) | FHE::Write(e) | FHE::Read(e) => e,
        }
    }

    pub fn into_inner(self) -> FileError<E> {
        use FileHandleError as FHE;
        match self {
            FHE::Create(e) | FHE::Write(e) | FHE::Read(e) => e,
        }
    }
}

impl<E: std::error::Error> From<FileError<E>> for FileErrorBuilder<E> {
    fn from(value: FileError<E>) -> Self {
        value.into_builder()
    }
}
