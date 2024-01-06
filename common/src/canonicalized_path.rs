use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct CanonicalizedPath {
    value: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    Creation(std::io::Error),
    Read(std::io::Error),
    Write(std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl CanonicalizedPath {
    pub fn new(path: &Path) -> Result<Self> {
        path.canonicalize()
            .map(|value| CanonicalizedPath { value })
            .map_err(Error::Creation)
    }

    pub fn to_str(&self) -> &str {
        self.value
            .to_str()
            .expect("Canonicalized strings must be valid OS strings")
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.value.to_path_buf()
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        std::fs::read(&self.value).map_err(Error::Read)
    }

    pub fn write_with_ensured_parent_dir(&self, content: Vec<u8>) -> Result<()> {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(
                self.value
                    .parent()
                    .expect("Canonicalized paths must have a parent"),
            )
            .and_then(|_| std::fs::write(&self.value, content))
            .map_err(Error::Write)
    }
}
