mod args;
mod command;
mod error;
mod files;
use args::Args;
use clap::Parser;
use command::Command;
use error::Result;

fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}

fn run() -> Result<()> {
    let args = init();

    let command = Command::try_from(args.command.as_str()).expect("Expected valid command string");
    let files = files::parse(&args.files);
    let outputs = command
        .run(files.iter().map(|path| path.as_path()).collect())
        .into_iter()
        .map(|result| result.unwrap());

    for (stdout, stderr) in outputs.into_iter() {
        print!("{}", stdout);
        eprint!("{}", stderr);
    }

    Ok(())
}

fn init() -> Args {
    Args::parse()
}
