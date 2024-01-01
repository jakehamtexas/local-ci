#![deny(missing_docs)]

use crate::files;
use clap::Parser;
use work_skipping::RunArgs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The list of files to operate on.
    #[arg()]
    pub files: Vec<String>,
    /// Command to run. Files are passed to the command through stdin, one at a time.
    #[arg(long)]
    pub command: String,
    /// A list of files that serve as the "cache buster" for the results cache. This means that if
    /// any of these files' contents change, the cache must be evicted for a given command and file.
    /// For instance, when using `eslint` on a list of JavaScript files, one of the cache key files
    /// for this operation may be the `.eslintrc.json`, because if it changes, previous runs of
    /// `eslint` against target files may reflect missing or out-of-date linter configuration.
    ///
    /// e.g. {cmd} source-1.js source-2.js --command eslint --cache-key-files .eslintrc.json
    #[arg(long)]
    pub cache_key_files: Option<Vec<String>>,
}

impl Into<RunArgs> for Args {
    fn into(self) -> RunArgs {
        let files = files::parse(&self.files);
        let cache_key_files = self
            .cache_key_files
            .map(|cache_key_files| files::parse(&cache_key_files));

        RunArgs {
            command: self.command,
            cache_key_files,
            files,
        }
    }
}
