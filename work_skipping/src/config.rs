use common::ReadonlyList;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, PartialEq)]
pub struct ArgsContext<'a> {
    command: &'a str,
    cache_key_files: Option<ReadonlyList<PathBuf>>,
    state_dir: &'a Path,
}

impl<'a> ArgsContext<'a> {
    pub fn new(
        command: &'a str,
        cache_key_files: Option<ReadonlyList<PathBuf>>,
        state_dir: &'a Path,
    ) -> Self {
        ArgsContext {
            command,
            cache_key_files,
            state_dir,
        }
    }

    pub fn command(&self) -> &str {
        self.command
    }

    pub fn cache_key_files(&self) -> Option<&ReadonlyList<PathBuf>> {
        self.cache_key_files.as_ref()
    }

    pub fn state_dir(&self) -> &Path {
        self.state_dir
    }
}

#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    context: ArgsContext<'a>,
    path: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(
        command: &'a str,
        cache_key_files: Option<ReadonlyList<PathBuf>>,
        state_dir: &'a Path,
        path: &'a Path,
    ) -> Arc<Config<'a>> {
        let context = ArgsContext::new(command, cache_key_files, state_dir);
        Arc::new(Config { context, path })
    }

    pub fn command(&self) -> &str {
        self.context.command()
    }

    pub fn cache_key_files(&self) -> Option<&ReadonlyList<PathBuf>> {
        self.context.cache_key_files()
    }

    pub fn state_dir(&self) -> &Path {
        self.context.state_dir()
    }

    pub fn path(&self) -> &'a Path {
        self.path
    }
}
