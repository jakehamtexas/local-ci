use common::prelude::*;
use std::sync::Arc;
mod cache;
mod command;
mod config;
mod error;
mod run_result;

pub use crate::config::{ArgsContext, Config};
pub use cache::{CacheReadError, CacheWriteError, FileId};
use cache::{FsCache, ReadCache, WriteCache};
use command::Command;
pub use command::CommandError;
use common::RelativePath;
pub use error::{Error, InnerError, Result};
pub use run_result::RunResult;

fn inner(config: Arc<Config>) -> error::Result<RunResult, error::InnerError> {
    let command =
        Command::try_from(config.command()).or(Err(error::InnerError::CommandCreation))?;

    let cache = FsCache::new(&config);
    let path = RelativePath::new(config.path());
    if let Some(run_result) = cache.read(&path)? {
        Ok(run_result)
    } else {
        let run_result = command.run(path)?;

        match cache.write(&run_result) {
            Ok(_) => Ok(run_result),
            Err(e) => {
                Err(error::InnerError::CacheWriteFailedButCommandExecutionSucceeded(run_result, e))
            }
        }
    }
}

pub fn run(args: Arc<Config>) -> error::Result<RunResult> {
    inner(Arc::clone(&args)).map_err(|e| error::Error::new(Arc::clone(&args), e))
}
