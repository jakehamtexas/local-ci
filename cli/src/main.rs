mod args;
mod cache;
mod canonicalized_path;
mod command;
mod error;
mod files;
use args::Args;
use cache::Cache;
use cache::FsCache;
use clap::Parser;
use command::Command;
use error::Result;

fn main() -> Result<()> {
    let args = init();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let command = Command::try_from(args.command.as_str())?;
    let files = files::parse(&args.files);
    let cache_key_files = args
        .cache_key_files
        .map(|cache_key_files| files::parse(&cache_key_files));

    let command_name = command.name();
    let cache = FsCache::new(None, command_name.as_str(), cache_key_files);
    let run_results = command
        .run(files.iter().map(|path| path.as_path()).collect(), &cache)
        .into_iter()
        .map(|result| result.unwrap());

    for run_result in run_results {
        cache.write(&run_result)?;
        run_result.print_stdout();
        run_result.print_stderr();
    }

    Ok(())
}

fn init() -> Args {
    Args::parse()
}
