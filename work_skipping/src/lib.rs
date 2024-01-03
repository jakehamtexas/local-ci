mod cache;
mod canonicalized_path;
mod command;
pub mod error;
mod run_result;

use cache::{Cache, FsCache};
use command::Command;
use common::ReadonlyList;
use error::FileErrorMap;
use std::{collections::HashMap, path::PathBuf};

pub struct RunArgs {
    pub command: String,
    pub cache_key_files: Option<ReadonlyList<PathBuf>>,
    pub files: ReadonlyList<PathBuf>,
}

pub fn run(args: RunArgs) -> error::Result<()> {
    let command =
        Command::try_from(args.command.as_str()).or(Err(error::Error::CommandCreation))?;
    let command_name = command.name();
    let cache = FsCache::new(None, command_name.as_str(), args.cache_key_files);
    let (oks, errs): (Vec<_>, Vec<_>) = command
        .run(
            args.files.iter().map(|path| path.as_path()).collect(),
            &cache,
        )
        .into_iter()
        .partition(Result::is_ok);

    if !errs.is_empty() {
        let map: FileErrorMap<command::Error> =
            errs.into_iter()
                .map(|r| r.unwrap_err())
                .fold(HashMap::new(), |mut acc, e| {
                    acc.insert(e.path().unwrap(), e);
                    acc
                });
        return Err(error::Error::CommandExecution(map));
    }

    let mut cache_write_errors: FileErrorMap<cache::Error> = HashMap::new();
    for run_result in oks.into_iter().map(|e| e.unwrap()) {
        if let Err(e) = cache.write(&run_result) {
            cache_write_errors.insert(run_result.path().to_owned(), e);
        }
        run_result.print_stdout();
        run_result.print_stderr();
    }

    Ok(())
}
