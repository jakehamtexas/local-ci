use crate::run_result::RunResult;
use common::canonicalized_path::CanonicalizedPath;
use std::rc::Rc;
pub mod error;
pub use self::error::{Error, Result};

#[derive(Debug)]
pub struct Command<'a> {
    name: &'a str,
    args: Rc<[&'a str]>,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = Error;
    fn try_from(value: &'a str) -> Result<Command<'a>, Self::Error> {
        let parts = value.split_whitespace();
        let parts = parts.collect::<Rc<_>>();
        let parts = parts.split_first();

        let (name, command_args) = parts.ok_or(Error::CreationFailed)?;

        Ok(Command {
            name,
            args: command_args.iter().map(|arg| arg.to_owned()).collect(),
        })
    }
}

impl Command<'_> {
    pub fn run(&self, path: CanonicalizedPath) -> Result<RunResult> {
        let output = std::process::Command::new(self.name)
            .args(self.args.iter())
            .arg(path.to_path_buf())
            .output()
            .map_err(Error::OutputIo)?;

        Ok(RunResult::new(&output, path.to_owned()))
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.name, self.args.join(" "))
    }
}
