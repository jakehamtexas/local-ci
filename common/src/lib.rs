use std::sync::Arc;
mod fs;
mod opaque_error;

pub use fs::{
    FileError, FileErrorBuilder, FileHandleError, FileHandleResult, FileResult, RelativePath,
};

pub use opaque_error::OpaqueError;

pub type ReadonlyList<T> = Arc<[T]>;

pub mod prelude {
    pub use super::fs::prelude::*;
}
