#![deny(missing_docs)]

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The list of files to operate on.
    #[arg()]
    pub files: Vec<String>,
    /// Command to run. Files are passed to the command through stdin.
    #[arg(long)]
    pub command: String,
}
