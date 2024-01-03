use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct CanonicalizedPath {
    pub value: PathBuf,
}

impl CanonicalizedPath {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        path.canonicalize().map(|value| CanonicalizedPath { value })
    }
}
