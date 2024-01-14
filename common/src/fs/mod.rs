mod error;
mod fs_facade;
mod relative_path;

pub use error::{FileError, FileErrorBuilder, FileHandleError, FileHandleResult, FileResult};
pub use relative_path::RelativePath;

pub mod prelude {
    pub use super::fs_facade::FsFacade;
}
