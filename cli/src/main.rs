mod args;
mod error;
mod files;
use args::Args;
use clap::Parser;
use std::path::PathBuf;
use work_skipping::{Result, RunArgs};

const DEFAULT_STATE_DIR_PREFIX: &str = ".local-ci";
const DEFAULT_STATE_DIR_BASENAME: &str = "cache";

fn main() {
    let args = init();

    let files = files::parse(&args.files);
    let cache_key_files = args
        .cache_key_files
        .map(|cache_key_files| files::parse(&cache_key_files));

    let state_dir = args.state_dir.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(DEFAULT_STATE_DIR_PREFIX).join(DEFAULT_STATE_DIR_BASENAME)
    });

    let run_args = files.iter().map(|file| RunArgs {
        command: &args.command,
        cache_key_files: cache_key_files.as_ref(),
        file,
        state_dir: &state_dir,
    });

    let (_, err_results): (_, Vec<_>) = run_args.map(run).partition(Result::is_ok);
    if let Err(e) = error::Error::new(err_results).into_result() {
        panic!("{}", e);
    }
}

fn run(run_args: RunArgs<'_>) -> Result<'_, ()> {
    let run_result = work_skipping::run(run_args)?;

    print!("{}", run_result.stdout());
    eprint!("{}", run_result.stderr());

    Ok(())
}

fn init() -> Args {
    Args::parse()
}
