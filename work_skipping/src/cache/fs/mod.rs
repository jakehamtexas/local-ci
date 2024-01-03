use crate::canonicalized_path::CanonicalizedPath;
use crate::run_result::RunResult;
use common::ReadonlyList;
use file_id::get_file_id;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use std::rc::Rc;
pub mod error;
mod file_id;
pub use self::error::{Error, Result};
use crate::cache::Cache;
pub use crate::cache::{Error as CacheError, Result as CacheResult};

pub struct FsCache<'a> {
    state_dir: PathBuf,
    command: &'a str,
    cache_key_file_paths: Option<ReadonlyList<CanonicalizedPath>>,
}

const DEFAULT_DIR_PREFIX: &'static str = ".local-ci";
fn get_state_dir(prefix: &str) -> PathBuf {
    Path::new(prefix).join("cache")
}

impl FsCache<'_> {
    pub fn new<'a>(
        dir_prefix_override: Option<&'a str>,
        command: &'a str,
        cache_key_file_paths: Option<Rc<[PathBuf]>>,
    ) -> FsCache<'a> {
        let dir_prefix = dir_prefix_override.unwrap_or(DEFAULT_DIR_PREFIX);
        FsCache {
            state_dir: get_state_dir(dir_prefix),
            command,
            cache_key_file_paths: cache_key_file_paths.map(|paths| {
                paths
                    .iter()
                    .filter_map(|path| CanonicalizedPath::new(path).ok())
                    .collect::<Rc<[CanonicalizedPath]>>()
            }),
        }
    }

    fn get_filename(&self, path: &CanonicalizedPath) -> Result<PathBuf> {
        get_file_id(self.command, path, self.cache_key_file_paths.as_ref()).map(|file_id| {
            self.state_dir
                .join(file_id.command_id)
                .join(file_id.cache_key_file_contents_id)
                .join(file_id.target_file_path_id)
        })
    }

    fn _write(&self, run_result: &RunResult) -> Result<()> {
        let path = self.get_filename(run_result.path())?;
        DirBuilder::new()
            .recursive(true)
            .create(path.parent().expect("Parent must exist"))?;
        let writable = serde_json::to_string(run_result)?.into_bytes();
        fs::write(path, writable)?;

        Ok(())
    }

    fn _read(&self, path: &CanonicalizedPath) -> Result<Option<RunResult>> {
        let buf = fs::read(self.get_filename(path)?).map_err(Error::from);

        match buf {
            Ok(buf) => {
                let run_result: RunResult = serde_json::from_slice(buf.as_slice())?;
                Ok(Some(run_result))
            }
            Err(Error::IoNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Cache for FsCache<'_> {
    fn write(&self, run_result: &RunResult) -> CacheResult<()> {
        Ok(self._write(run_result)?)
    }

    fn read(&self, path: &CanonicalizedPath) -> CacheResult<Option<RunResult>> {
        Ok(self._read(path)?)
    }
}
