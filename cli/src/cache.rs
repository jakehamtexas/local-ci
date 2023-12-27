use std::fs::{self, DirBuilder};
use std::path::Path;
use xxhash_rust::xxh3::xxh3_64;

use crate::command::RunResult;

#[derive(Debug)]
pub enum Error {
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

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait Cache {
    fn write(&self, run_result: &RunResult) -> Result<()>;
}

pub struct FsCache<'a> {
    dir_prefix: &'a str,
}

const DEFAULT_DIR_PREFIX: &'static str = ".local-ci/cache";
const DEFAULT_FS_CACHE: FsCache = FsCache {
    dir_prefix: DEFAULT_DIR_PREFIX,
};

impl Default for FsCache<'_> {
    fn default() -> Self {
        DEFAULT_FS_CACHE
    }
}

impl FsCache<'_> {
    pub const fn new<'a>(dir_prefix_override: Option<&'a str>) -> FsCache<'a> {
        if let Some(dir_prefix) = dir_prefix_override {
            FsCache { dir_prefix }
        } else {
            DEFAULT_FS_CACHE
        }
    }
}

fn get_file_id(run_result: &RunResult) -> Result<u64> {
    let full_path = run_result.path().expect("Canonicalized path should exist");
    Ok(xxh3_64(
        full_path
            .to_str()
            .expect("Canonical paths are supposed to be coercible to str")
            .as_bytes(),
    ))
}

impl Cache for FsCache<'_> {
    fn write(&self, run_result: &RunResult) -> Result<()> {
        let state_dir = Path::new(self.dir_prefix);
        let file_id = get_file_id(&run_result)?;

        DirBuilder::new().recursive(true).create(state_dir)?;
        let writable = serde_json::to_string(run_result)?.into_bytes();
        fs::write(
            state_dir
                .with_file_name(file_id.to_string())
                .with_extension("cache"),
            writable,
        )?;

        Ok(())
    }
}

