use common::canonicalized_path::CanonicalizedPath;
use serde::Deserialize;
use serde::Serialize;

use std::process::Output;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
    stdout: String,
    stderr: String,
    canonicalized_path: CanonicalizedPath,
}

impl RunResult {
    pub fn new(output: &Output, path: CanonicalizedPath) -> RunResult {
        RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            canonicalized_path: path,
        }
    }

    pub fn path(&self) -> &CanonicalizedPath {
        &self.canonicalized_path
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}
