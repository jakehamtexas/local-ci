use super::error::FileHandleResult;
use super::fs_facade::{FsFacade, FsFacadeData};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct RelativePath(FsFacadeData);

impl RelativePath {
    pub fn to_path_buf(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

impl FromStr for RelativePath {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        Ok(Self::new(&path))
    }
}

impl std::fmt::Display for RelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().display())
    }
}

impl FsFacade for RelativePath {
    fn new(path: &Path) -> Self {
        RelativePath(FsFacadeData::new(path))
    }

    fn read(&self) -> FileHandleResult<Vec<u8>> {
        self.0.read()
    }

    fn write(&self, content: Vec<u8>) -> FileHandleResult<()> {
        self.0.write(content)
    }
}
