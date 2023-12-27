mod args;
mod cache;
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
    run()
}

fn run() -> Result<()> {
    let args = init();

    let command = Command::try_from(args.command.as_str()).expect("Expected valid command string");
    let files = files::parse(&args.files);
    let run_results = command
        .run(files.iter().map(|path| path.as_path()).collect())
        .into_iter()
        .map(|result| result.unwrap());

    let cache = FsCache::new(None);
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
