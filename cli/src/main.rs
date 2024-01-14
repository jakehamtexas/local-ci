mod args;
mod error;
mod files;
use args::Args;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use work_skipping::{Config, Result};

const DEFAULT_STATE_DIR_PREFIX: &str = ".local-ci";
const DEFAULT_STATE_DIR_BASENAME: &str = "cache";

fn main() {
    let args = init();

    let files = files::parse(&args.files);
    let cache_key_files = args
        .cache_key_files
        .as_ref()
        .map(|cache_key_files| files::parse(cache_key_files));

    let state_dir = args.state_dir.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(DEFAULT_STATE_DIR_PREFIX).join(DEFAULT_STATE_DIR_BASENAME)
    });

    let run_args = files
        .iter()
        .map(|file| {
            let config = Config::new(
                &args.command,
                cache_key_files.as_ref().cloned(),
                &state_dir,
                file,
            );
            run(config)
        })
        .partition(Result::is_ok);

    let (_, err_results): (_, Vec<_>) = run_args;
    if let Err(e) = error::Error::new(err_results).into_result() {
        panic!("{}", e);
    }
}

fn run(config: Arc<Config>) -> Result<()> {
    let run_result = work_skipping::run(config)?;

    print!("{}", run_result.stdout());
    eprint!("{}", run_result.stderr());

    Ok(())
}

fn init() -> Args {
    Args::parse()
}
