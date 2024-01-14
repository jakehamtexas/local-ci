use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

use super::error::{FileErrorBuilder, FileHandleError, FileHandleResult};

pub trait FsFacade
where
    Self: Sized,
{
    // TODO: Add base_path argument to new
    fn new(path: &Path) -> Self;
    fn write(&self, content: Vec<u8>) -> FileHandleResult<()>;
    fn read(&self) -> FileHandleResult<Vec<u8>>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct FsFacadeData {
    path: PathBuf,
    base_path: PathBuf,
}

impl FsFacadeData {
    pub fn to_path_buf(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

fn ensure_dir(path: Option<&Path>) -> std::io::Result<()> {
    match path {
        Some(path) if !path.exists() => std::fs::DirBuilder::new().recursive(true).create(path),
        // Either path exists or is None (the path is the current directory)
        _ => Ok(()),
    }
}

fn write_with_ensured_parent_dir(path: &Path, content: &[u8]) -> FileHandleResult<()> {
    let new_creation_err = |msg: &'static str| {
        move |e: std::io::Error| {
            FileHandleError::Create(
                FileErrorBuilder::new()
                    .with_path(path.to_path_buf())
                    .with_message(msg)
                    .with_source(e)
                    .build(),
            )
        }
    };
    let parent = path.parent();

    ensure_dir(parent).map_err(new_creation_err("Failed to create dir at path's parent"))?;

    let mut file = std::fs::File::create(path).map_err(new_creation_err("Failed writing path"))?;
    file.write_all(content)
        .map_err(new_creation_err("Failed to write to file once created"))
}

impl FsFacade for FsFacadeData {
    fn new(path: &Path) -> Self {
        FsFacadeData {
            path: path.to_path_buf(),
            base_path: PathBuf::from("./"),
        }
    }

    fn read(&self) -> FileHandleResult<Vec<u8>> {
        std::fs::read(&self.path).map_err(|e| {
            FileHandleError::Read(
                FileErrorBuilder::new()
                    .with_source(e)
                    .with_path(self.path.clone())
                    .build(),
            )
        })
    }

    fn write(&self, content: Vec<u8>) -> FileHandleResult<()> {
        write_with_ensured_parent_dir(&self.path, &content)
    }
}
