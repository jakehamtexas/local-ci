mod base;
pub mod error;
mod fs;

use self::error::{CacheReadError as CacheReadErrorBase, CacheWriteError as CacheWriteErrorBase};
pub use base::{ReadCache, WriteCache};
use fs::FsCacheError;
pub use fs::{FileId, FsCache};

// TODO: Configure conditional compilation of the resultant error types,
//       inclusion of fs module, etc., so that other cache implementations
//       can be selected at compile time
pub type CacheReadError = CacheReadErrorBase<FsCacheError>;
pub type CacheWriteError = CacheWriteErrorBase<FsCacheError>;

pub type ReadResult<T, E = CacheReadError> = std::result::Result<T, E>;
pub type WriteResult<T, E = CacheWriteError> = std::result::Result<T, E>;

impl From<FsCacheError> for CacheReadError {
    fn from(value: FsCacheError) -> Self {
        CacheReadError::Other(Box::new(value))
    }
}

impl From<FsCacheError> for CacheWriteError {
    fn from(value: FsCacheError) -> Self {
        CacheWriteError::Other(Box::new(value))
    }
}
