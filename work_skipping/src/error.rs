use crate::cache;
use crate::command;
use crate::RunResult;
use common::canonicalized_path;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub struct Error<'a> {
    path: &'a Path,
    inner: InnerError,
}

impl<'a> Error<'a> {
    pub fn new(path: &'a Path, inner: InnerError) -> Self {
        Error { path, inner }
    }

    pub fn kind(&self) -> &InnerError {
        &self.inner
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}, error: {:#?}",
            self.path.to_str().unwrap_or("None"),
            self.inner
        )
    }
}

#[derive(Debug)]
pub enum InnerError {
    CommandCreation,
    BadPath(canonicalized_path::Error),
    CommandExecution(command::Error),
    CacheRead(cache::ReadError),
    CacheWriteFailedButCommandExecutionSucceeded(RunResult, cache::WriteError),
}

impl From<command::Error> for InnerError {
    fn from(value: command::Error) -> Self {
        match value {
            command::Error::CreationFailed => InnerError::CommandCreation,
            command::Error::OutputIo(_) => InnerError::CommandExecution(value),
        }
    }
}

impl From<cache::ReadError> for InnerError {
    fn from(value: cache::ReadError) -> Self {
        InnerError::CacheRead(value)
    }
}

pub type Result<'a, T, E = Error<'a>> = std::result::Result<T, E>;
