use crate::cache::Cache;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Output;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Cache(crate::cache::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<crate::cache::Error> for Error {
    fn from(value: crate::cache::Error) -> Self {
        Error::Cache(value)
    }
}

pub type Result<'a, T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Command<'a> {
    name: &'a str,
    args: Rc<[&'a str]>,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Command<'a>, Self::Error> {
        let parts = value.split_whitespace();
        let parts = parts.collect::<Rc<_>>();
        let parts = parts.split_first();

        let (name, command_args) = parts.ok_or(())?;

        Ok(Command {
            name,
            args: command_args.iter().map(|arg| arg.to_owned()).collect(),
        })
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
    stdout: String,
    stderr: String,
    original_path: PathBuf,
    canonicalized_path: Option<CanonicalizedPath>,
}

impl RunResult {
    fn new(output: &Output, path: &Path) -> RunResult {
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

impl Command<'_> {
    pub fn run(&self, paths: Rc<[&Path]>, cache: &dyn Cache) -> Vec<Result<RunResult>> {
        paths
            .iter()
            .map(|path| {
                let run_result = if let Some(run_result) = cache.read(path)? {
                    run_result
                } else {
                    let output = std::process::Command::new(self.name)
                        .args(self.args.iter())
                        .arg(path)
                        .output()?;

                    RunResult::new(&output, path)
                };

                Ok(run_result)
            })
            .collect()
    }
}
