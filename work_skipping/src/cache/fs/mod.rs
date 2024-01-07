use crate::run_result::RunResult;
use common::canonicalized_path::CanonicalizedPath;
use common::ReadonlyList;
use file_id::get_file_id;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
pub mod error;
mod file_id;
pub use self::error::{Error, Result};
use crate::cache::{ReadCache, ReadResult, WriteCache, WriteResult};

pub struct FsCache<'a> {
    state_dir: &'a Path,
    command: &'a str,
    cache_key_file_paths: Option<ReadonlyList<CanonicalizedPath>>,
}

impl FsCache<'_> {
    pub fn new<'a>(
        state_dir: &'a Path,
        command: &'a str,
        cache_key_file_paths: Option<&'a Rc<[PathBuf]>>,
    ) -> FsCache<'a> {
        FsCache {
            state_dir,
            command,
            cache_key_file_paths: cache_key_file_paths.map(|paths| {
                paths
                    .iter()
                    .filter_map(|path| CanonicalizedPath::new(path).ok())
                    .collect::<Rc<[CanonicalizedPath]>>()
            }),
        }
    }

    fn get_filename(&self, path: &CanonicalizedPath) -> Result<CanonicalizedPath> {
        let file_id = get_file_id(self.command, path, self.cache_key_file_paths.as_ref())?;
        let path = CanonicalizedPath::new(
            self.state_dir
                .join(file_id.command_id)
                .join(file_id.cache_key_file_contents_id)
                .join(file_id.target_file_path_id)
                .as_path(),
        )?;

        Ok(path)
    }
}

impl ReadCache for FsCache<'_> {
    fn read(&self, path: &CanonicalizedPath) -> ReadResult<Option<RunResult>> {
        let buf = self.get_filename(path)?.read().map_err(|e| e.into());

        match buf {
            Ok(buf) => {
                let run_result: RunResult = serde_json::from_slice(buf.as_slice())?;
                Ok(Some(run_result))
            }
            Err(Error::IoNotFound) => Ok(None),
            Err(e) => Err(e)?,
        }
    }
}

impl WriteCache for FsCache<'_> {
    fn write(&self, run_result: &RunResult) -> WriteResult<()> {
        let path = self.get_filename(run_result.path())?;
        let writable = serde_json::to_string(run_result)?.into_bytes();
        path.write_with_ensured_parent_dir(writable)?;

        Ok(())
    }
}
