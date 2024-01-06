mod cache;
mod command;
mod error;
mod run_result;

use cache::{FsCache, ReadCache, WriteCache};
pub use cache::{ReadError as CacheReadError, WriteError as CacheWriteError};
use command::Command;
pub use command::Error as CommandError;
use common::canonicalized_path::CanonicalizedPath;
use common::ReadonlyList;
pub use error::{Error, InnerError, Result};
use run_result::RunResult;
use std::path::{Path, PathBuf};

pub struct RunArgs<'a> {
    pub command: &'a str,
    pub cache_key_files: Option<&'a ReadonlyList<PathBuf>>,
    pub file: &'a Path,
}

fn inner(args: RunArgs) -> error::Result<RunResult, error::InnerError> {
    let command = Command::try_from(args.command).or(Err(error::InnerError::CommandCreation))?;
    let command_name = command.name();
    let cache = FsCache::new(None, command_name.as_str(), args.cache_key_files);
    let path = CanonicalizedPath::new(args.file).map_err(error::InnerError::BadPath)?;
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

pub fn run(args: RunArgs) -> error::Result<RunResult> {
    let file = args.file;
    inner(args).map_err(|e| error::Error::new(file, e))
}
