use crate::cache;
use crate::cache::CacheReadError;
use crate::cache::CacheWriteError;
use crate::command::CommandError;
use crate::Config;
use crate::RunResult;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[error("{:#?}", self)]
pub struct Error<'a> {
    config: std::sync::Arc<Config<'a>>,
    #[source]
    inner: InnerError,
}

impl<'a> Error<'a> {
    pub fn new(config: std::sync::Arc<Config<'a>>, inner: InnerError) -> Self {
        Error { config, inner }
    }

    pub fn kind(&self) -> &InnerError {
        &self.inner
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum InnerError {
    #[error("Command creation failed")]
    CommandCreation,
    #[error("Command execution failed: {0:?}")]
    CommandExecution(CommandError),
    #[error("Cache read failed: {0:?}")]
    CacheRead(CacheReadError),
    #[error(
        "Cache write failed, but execution succeeded. RunResult: {:?}, Error: {:?}",
        0,
        1
    )]
    CacheWriteFailedButCommandExecutionSucceeded(RunResult, CacheWriteError),
}

impl From<CommandError> for InnerError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::CreationFailed => InnerError::CommandCreation,
            CommandError::OutputIo(_, _) => InnerError::CommandExecution(value),
        }
    }
}

impl From<cache::CacheReadError> for InnerError {
    fn from(value: cache::CacheReadError) -> Self {
        InnerError::CacheRead(value)
    }
}

pub type Result<'a, T, E = Error<'a>> = std::result::Result<T, E>;
