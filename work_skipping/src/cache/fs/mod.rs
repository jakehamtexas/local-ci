use crate::run_result::RunResult;
use crate::Config;
use common::FileHandleResult;
use common::ReadonlyList;
use common::RelativePath;
use std::path::Path;
use std::sync::Arc;
pub mod error;
mod file_id;
pub use self::error::FsCacheError;
pub use self::file_id::FileId;
use crate::cache::{ReadCache, ReadResult, WriteCache, WriteResult};
use common::prelude::*;

pub struct FsCache<'a> {
    command: &'a str,
    state_dir: &'a Path,
    cache_key_files: Option<ReadonlyList<RelativePath>>,
}

impl FsCache<'_> {
    pub fn new<'a>(config: &'a Arc<Config>) -> FsCache<'a> {
        FsCache {
            command: config.command(),
            state_dir: config.state_dir(),
            cache_key_files: config.cache_key_files().map(|files| {
                files
                    .iter()
                    .map(|file| RelativePath::new(file.as_path()))
                    .collect()
            }),
        }
    }

    // TODO: Explore whether we always use the path that is local to the config
    //       If so, get rid of the path argument
    fn get_filename(&self, path: &RelativePath) -> FileHandleResult<RelativePath> {
        FileId::new(
            self.command,
            path.as_path().as_os_str(),
            self.cache_key_files.as_ref(),
        )
        .map(|file_id| {
            RelativePath::new(
                self.state_dir
                    .join(file_id.command_id)
                    .join(file_id.cache_key_file_contents_id)
                    .join(file_id.target_file_path_id)
                    .as_path(),
            )
        })
    }
}

impl ReadCache for FsCache<'_> {
    fn read(&self, path: &RelativePath) -> ReadResult<Option<RunResult>> {
        let buf = self
            .get_filename(path)
            .and_then(|file| file.read())
            .map_err(FsCacheError::from_error_with_path(path.to_path_buf()));

        match buf {
            Ok(buf) => {
                let run_result: RunResult = serde_json::from_slice(buf.as_slice())?;
                Ok(Some(run_result))
            }
            Err(FsCacheError::IoNotFound(_)) => Ok(None),
            Err(e) => Err(e)?,
        }
    }
}

impl WriteCache for FsCache<'_> {
    fn write(&self, run_result: &RunResult) -> WriteResult<()> {
        let path = run_result.path();
        let with_path = FsCacheError::from_error_with_path(path.to_path_buf());

        let writable = serde_json::to_string(run_result)?.into_bytes();

        self.get_filename(path)
            .and_then(|file| file.write(writable))
            .map_err(with_path)?;

        Ok(())
    }
}
