use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use xxhash_rust::xxh3::xxh3_64;

use crate::command::{CanonicalizedPath, RunResult};

#[derive(Debug)]
pub enum Error {
    Other(Option<String>),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serialization(value)
    }
}

pub type Result<'a, T, E = Error> = std::result::Result<T, E>;

pub trait Cache {
    fn write(&self, run_result: &RunResult) -> Result<()>;
    fn read(&self, path: &Path) -> Result<Option<RunResult>>;
}

pub struct FsCache {
    state_dir: PathBuf,
}

const DEFAULT_DIR_PREFIX: &'static str = ".local-ci";
fn get_state_dir(prefix: &str) -> PathBuf {
    Path::new(prefix).join("cache")
}

impl Default for FsCache {
    fn default() -> Self {
        FsCache {
            state_dir: get_state_dir(DEFAULT_DIR_PREFIX),
        }
    }
}

impl FsCache {
    pub fn new(dir_prefix_override: Option<&str>) -> FsCache {
        if let Some(dir_prefix) = dir_prefix_override {
            FsCache {
                state_dir: get_state_dir(dir_prefix),
            }
        } else {
            FsCache::default()
        }
    }

    fn get_filename(&self, path: Option<&CanonicalizedPath>) -> Result<PathBuf> {
        let file_id = get_file_id(path)?;
        Ok(self
            .state_dir
            .with_file_name(file_id.to_string())
            .with_extension("cache"))
    }
}

fn get_file_id(canonicalized_path: Option<&CanonicalizedPath>) -> Result<u64> {
    Ok(xxh3_64(
        canonicalized_path
            .ok_or(Error::Other(Some(
                "Canonicalized path does not exist".to_string(),
            )))?
            .value
            .to_str()
            .expect("Canonical paths are supposed to be coercible to str")
            .as_bytes(),
    ))
}

impl Cache for FsCache {
    fn write(&self, run_result: &RunResult) -> Result<()> {
        DirBuilder::new().recursive(true).create(&self.state_dir)?;
        let writable = serde_json::to_string(run_result)?.into_bytes();
        fs::write(self.get_filename(run_result.path())?, writable)?;

        Ok(())
    }

    fn read(&self, path: &Path) -> Result<Option<RunResult>> {
        let path = &CanonicalizedPath::new(path);

        let buf = fs::read(self.get_filename(path.as_ref())?);

        match buf {
            Ok(buf) => {
                let run_result: RunResult = serde_json::from_slice(buf.as_slice())?;
                Ok(Some(run_result))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e)?,
        }
    }
}

