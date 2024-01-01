mod args;
mod files;
use args::Args;
use clap::Parser;

fn main() -> work_skipping::error::Result<()> {
    let args = init();

    work_skipping::run(args.into())
}

fn init() -> Args {
    Args::parse()
}
