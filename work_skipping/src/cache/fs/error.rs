use common::{FileError, FileHandleError};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FsCacheError {
    #[error("Not found: {0:?}")]
    IoNotFound(PathBuf),
    #[error("Unexpected FS Cache Error: {0:?}")]
    Io(#[source] FileError),
}

impl FsCacheError {
    pub fn from_error_with_path(path: PathBuf) -> impl FnOnce(FileHandleError) -> Self {
        move |e: FileHandleError| match e.inner().source().kind() {
            std::io::ErrorKind::NotFound => FsCacheError::IoNotFound(path),
            _ => FsCacheError::Io(e.into_inner()),
        }
    }
}
