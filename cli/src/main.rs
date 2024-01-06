mod args;
mod error;
mod files;
use args::Args;
use clap::Parser;
use work_skipping::{Result, RunArgs};

fn main() {
    let args = init();

    let files = files::parse(&args.files);
    let cache_key_files = args
        .cache_key_files
        .map(|cache_key_files| files::parse(&cache_key_files));

    let run_args = files.iter().map(|file| RunArgs {
        command: &args.command,
        cache_key_files: cache_key_files.as_ref(),
        file,
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
