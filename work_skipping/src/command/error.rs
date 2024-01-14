use common::RelativePath;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command creation failed")]
    CreationFailed,
    #[error("Output I/O failed: {0:?} {1:?}")]
    OutputIo(RelativePath, #[source] std::io::Error),
}

fn get_comparable(e: &CommandError) -> Option<std::io::ErrorKind> {
    match e {
        CommandError::CreationFailed => None,
        CommandError::OutputIo(_, e) => Some(e.kind()),
    }
}

impl PartialEq for CommandError {
    fn eq(&self, other: &Self) -> bool {
        let first = get_comparable(self);
        let second = get_comparable(other);

        (first.is_none() && second.is_none())
            || first.is_some_and(|first| second.is_some_and(|second| first == second))
    }
}

pub type CommandResult<T, E = CommandError> = std::result::Result<T, E>;
