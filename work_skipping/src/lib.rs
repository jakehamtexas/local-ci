mod cache;
mod canonicalized_path;
mod command;
pub mod error;
mod run_result;

use cache::{Cache, FsCache};
use command::Command;
use common::ReadonlyList;
use std::path::PathBuf;

pub struct RunArgs {
    pub command: String,
    pub cache_key_files: Option<ReadonlyList<PathBuf>>,
    pub files: ReadonlyList<PathBuf>,
}

pub fn run(args: RunArgs) -> error::Result<()> {
    let command = Command::try_from(args.command.as_str())?;
    let command_name = command.name();
    let cache = FsCache::new(None, command_name.as_str(), args.cache_key_files);
    let run_results = command
        .run(
            args.files.iter().map(|path| path.as_path()).collect(),
            &cache,
        )
        .into_iter()
        .map(|result| result.unwrap());

    for run_result in run_results {
        cache.write(&run_result)?;
        run_result.print_stdout();
        run_result.print_stderr();
    }

    Ok(())
}
