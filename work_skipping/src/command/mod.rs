use crate::run_result::RunResult;
use common::ReadonlyList;
use common::RelativePath;
mod error;
pub use self::error::{CommandError, CommandResult};

#[derive(Debug)]
pub struct Command<'a> {
    name: &'a str,
    args: ReadonlyList<&'a str>,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = CommandError;
    fn try_from(value: &'a str) -> CommandResult<Command<'a>, Self::Error> {
        let parts = value.split_whitespace();
        let parts = parts.collect::<ReadonlyList<_>>();
        let parts = parts.split_first();

        let (name, command_args) = parts.ok_or(CommandError::CreationFailed)?;

        Ok(Command {
            name,
            args: command_args.iter().map(|arg| arg.to_owned()).collect(),
        })
    }
}

impl Command<'_> {
    pub fn run(&self, path: RelativePath) -> CommandResult<RunResult> {
        let output = std::process::Command::new(self.name)
            .args(self.args.iter())
            .arg(path.to_path_buf())
            .output()
            .map_err(|e| CommandError::OutputIo(path.clone(), e))?;
        println!("{:?}", &path);

        Ok(RunResult::new(&output, path.to_owned()))
    }
}
