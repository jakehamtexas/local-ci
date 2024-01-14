use common::RelativePath;
use serde::Deserialize;
use serde::Serialize;

use std::process::Output;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunResult {
    stdout: String,
    stderr: String,
    path: RelativePath,
}

impl RunResult {
    pub fn new(output: &Output, path: RelativePath) -> RunResult {
        RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            path,
        }
    }

    pub fn path(&self) -> &RelativePath {
        &self.path
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}
