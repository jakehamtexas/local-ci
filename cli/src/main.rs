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

    let cache = FsCache::new(None);
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
