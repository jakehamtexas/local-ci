mod base;
pub mod error;
mod fs;
use common::canonicalized_path;

use self::error::{ReadError as ReadErrorBase, WriteError as WriteErrorBase};
pub use base::{ReadCache, WriteCache};
pub use fs::FsCache;

// TODO: Configure conditional compilation of the resultant error types,
//       inclusion of fs module, etc., so that other cache implementations
//       can be selected at compile time
pub type ReadError = ReadErrorBase<fs::Error>;
pub type WriteError = WriteErrorBase<fs::Error>;

pub type ReadResult<T, E = ReadError> = std::result::Result<T, E>;
pub type WriteResult<T, E = WriteError> = std::result::Result<T, E>;

impl From<fs::Error> for ReadError {
    fn from(value: fs::Error) -> Self {
        ReadError::Other(value)
    }
}

impl From<canonicalized_path::Error> for ReadError {
    fn from(value: canonicalized_path::Error) -> Self {
        ReadError::Other(value.into())
    }
}

impl From<fs::Error> for WriteError {
    fn from(value: fs::Error) -> Self {
        WriteError::Other(value)
    }
}

impl From<canonicalized_path::Error> for WriteError {
    fn from(value: canonicalized_path::Error) -> Self {
        WriteError::Other(value.into())
    }
}
