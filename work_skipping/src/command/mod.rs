use crate::cache::Cache;
use crate::run_result::RunResult;
use std::path::Path;
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

    pub fn name(&self) -> String {
        format!("{} {}", self.name, self.args.join(" "))
    }
}
