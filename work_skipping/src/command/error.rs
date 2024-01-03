use crate::canonicalized_path::CanonicalizedPath;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CreationFailed,
    BadPath(PathBuf, std::io::Error),
    OutputIo(CanonicalizedPath, std::io::Error),
    CacheRead(CanonicalizedPath, crate::cache::Error),
}

impl Error {
    pub fn path(&self) -> Option<CanonicalizedPath> {
        match self {
            Error::CreationFailed | Error::BadPath(_, _) => None,
            Error::OutputIo(p, _) | Error::CacheRead(p, _) => Some(p.to_owned()),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
