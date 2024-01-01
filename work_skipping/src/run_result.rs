use crate::canonicalized_path::CanonicalizedPath;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use std::process::Output;

use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
    stdout: String,
    stderr: String,
    original_path: PathBuf,
    canonicalized_path: Option<CanonicalizedPath>,
}

impl RunResult {
    pub fn new(output: &Output, path: &Path) -> RunResult {
        RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            original_path: path.to_path_buf(),
            canonicalized_path: CanonicalizedPath::new(path),
        }
    }

    pub fn print_stdout(&self) -> () {
        print!("{}", self.stdout);
    }

    pub fn print_stderr(&self) -> () {
        eprint!("{}", self.stdout);
    }

    pub fn path(&self) -> Option<&CanonicalizedPath> {
        self.canonicalized_path.as_ref()
    }
}
