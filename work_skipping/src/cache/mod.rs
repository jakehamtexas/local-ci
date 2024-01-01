mod base;
pub mod error;
mod fs;
pub use self::error::{Error, Result};
pub use base::Cache;
pub use fs::FsCache;
