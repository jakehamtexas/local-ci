use common::ReadonlyList;
use common::RelativePath;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;
use work_skipping::ArgsContext;
use work_skipping::FileId;

use work_skipping::Config;

fn get_default_run_args() -> &'static ArgsContext<'static> {
    static STATE_DIR_ONCE_LOCK: OnceLock<PathBuf> = OnceLock::new();
    static CONTEXT_ONCE_LOCK: OnceLock<ArgsContext<'static>> = OnceLock::new();

    let state_dir = STATE_DIR_ONCE_LOCK.get_or_init(|| PathBuf::from("./test/cache"));

    CONTEXT_ONCE_LOCK.get_or_init(|| ArgsContext::new("echo".into(), None, state_dir))
}

fn with_base_path<'a>(base_path: &'a Path, paths: &ReadonlyList<PathBuf>) -> ReadonlyList<PathBuf> {
    paths
        .iter()
        .map(|path| base_path.join(path))
        .collect::<ReadonlyList<_>>()
}

#[derive(Debug)]
pub struct ConfigBuilder<'a> {
    command: &'a str,
    cache_key_files: Option<&'a ReadonlyList<PathBuf>>,
    state_dir: &'a Path,
    base_path: &'a Path,
    path: Option<&'a Path>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new(base_path: &'a Path) -> ConfigBuilder<'a> {
        let default_run_args = get_default_run_args();

        ConfigBuilder {
            command: &default_run_args.command(),
            cache_key_files: None,
            state_dir: &default_run_args.state_dir(),
            base_path,
            path: None,
        }
    }

    pub fn with_command(mut self, command: &'a str) -> ConfigBuilder {
        self.command = command;
        self
    }

    pub fn with_cache_key_files(mut self, files: &'a ReadonlyList<PathBuf>) -> ConfigBuilder<'a> {
        self.cache_key_files = Some(files);
        self
    }

    pub fn with_state_dir(mut self, state_dir: &'a Path) -> ConfigBuilder {
        self.state_dir = state_dir;
        self
    }

    pub fn with_path(mut self, path: &'a Path) -> ConfigBuilder {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> Arc<Config<'a>> {
        let default_run_args = get_default_run_args();
        let cache_key_files = self
            .cache_key_files
            .or(default_run_args.cache_key_files())
            .map(|files| with_base_path(&self.base_path, files));

        Config::new(
            self.command,
            cache_key_files,
            self.state_dir,
            self.path.expect("Must have called with_path"),
        )
    }
}

#[derive(Debug, Default)]
pub struct FakeFileIdBuilder<'a> {
    command: Option<&'a str>,
    path: Option<&'a OsStr>,
    cache_key_files: Option<ReadonlyList<RelativePath>>,
}

impl<'a> FakeFileIdBuilder<'a> {
    pub fn with_command(mut self, command: &'a str) -> Self {
        self.command = Some(command);
        self
    }

    pub fn with_path(mut self, path: &'a OsStr) -> Self {
        self.path = Some(path);
        self
    }

    pub fn _with_cache_key_files(
        mut self,
        cache_key_files: Option<ReadonlyList<RelativePath>>,
    ) -> Self {
        self.cache_key_files = cache_key_files;
        self
    }

    pub fn build(self) -> FileId {
        let defaults = get_default_run_args();
        FileId::new(
            self.command.unwrap_or(defaults.command()),
            self.path.unwrap_or(OsStr::new("")),
            self.cache_key_files.as_ref(),
        )
        .unwrap()
    }
}
