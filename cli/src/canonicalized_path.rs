use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct CanonicalizedPath {
    pub value: PathBuf,
}

impl CanonicalizedPath {
    pub fn new(path: &Path) -> Option<Self> {
        path.canonicalize()
            .map(|value| CanonicalizedPath { value })
            .ok()
    }
}
