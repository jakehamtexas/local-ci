use crate::cache;
use crate::canonicalized_path::CanonicalizedPath;
use crate::command;
use core::fmt;
use std::collections::HashMap;

pub type FileErrorMap<T> = HashMap<CanonicalizedPath, T>;

fn displayable<T: std::fmt::Debug>(map: &FileErrorMap<T>) -> Vec<String> {
    map.into_iter()
        .map(|(path, e)| {
            format!(
                "Path: {}, error: {:#?}",
                path.value.to_str().unwrap_or("None"),
                e
            )
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub enum Error {
    CommandCreation,
    CommandExecution(FileErrorMap<command::Error>),
    CacheWrite(FileErrorMap<cache::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandCreation => write!(f, "Failed to create command due to parsing error."),
            Error::CommandExecution(map) => {
                write!(f, "Command execution failed: {:#?}", displayable(map))
            }
            Error::CacheWrite(map) => {
                write!(f, "Cache write failed: {:#?}", displayable(map))
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
