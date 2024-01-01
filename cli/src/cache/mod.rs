use crate::cache::file_id::get_file_id;
use crate::canonicalized_path::CanonicalizedPath;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use std::rc::Rc;
pub mod error;
mod file_id;
pub use self::error::{Error, Result};

use crate::command::RunResult;

pub trait Cache {
    fn write(&self, run_result: &RunResult) -> Result<()>;
    fn read(&self, path: &Path) -> Result<Option<RunResult>>;
}

pub struct FsCache<'a> {
    state_dir: PathBuf,
    command: &'a str,
    cache_key_file_paths: Option<Rc<[CanonicalizedPath]>>,
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
                    .filter_map(|path| CanonicalizedPath::new(path))
                    .collect::<Rc<[CanonicalizedPath]>>()
            }),
        }
    }

    fn get_filename(&self, path: Option<&CanonicalizedPath>) -> Result<PathBuf> {
        let file_id = get_file_id(
            self.command,
            path.expect("The target canonicalized path must exist."),
            self.cache_key_file_paths.as_ref(),
        )?;
        Ok(self
            .state_dir
            .join(file_id.command_id)
            .join(file_id.cache_key_file_contents_id)
            .join(file_id.target_file_path_id))
    }
}

impl Cache for FsCache<'_> {
    fn write(&self, run_result: &RunResult) -> Result<()> {
        let path = self.get_filename(run_result.path())?;
        DirBuilder::new()
            .recursive(true)
            .create(path.parent().expect("Parent must exist"))?;
        let writable = serde_json::to_string(run_result)?.into_bytes();
        fs::write(path, writable)?;

        Ok(())
    }

    fn read(&self, path: &Path) -> Result<Option<RunResult>> {
        let path = &CanonicalizedPath::new(path);

        let buf = fs::read(self.get_filename(path.as_ref())?).map_err(Error::from);

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
