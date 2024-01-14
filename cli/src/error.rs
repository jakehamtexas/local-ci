use common::ReadonlyList;
use core::fmt;
use std::sync::Arc;
#[derive(Debug)]

pub struct Error<'a> {
    command_creation_errs: Vec<work_skipping::Error<'a>>,
    bad_path_errs: Vec<work_skipping::Error<'a>>,
    cache_read_errs: Vec<work_skipping::Error<'a>>,
    command_execution_errs: Vec<work_skipping::Error<'a>>,
    cache_write_errs: Vec<work_skipping::Error<'a>>,
}

impl Error<'_> {
    pub fn new(err_results: Vec<Result<(), work_skipping::Error>>) -> Error {
        let mut error = Error {
            command_creation_errs: Vec::new(),
            bad_path_errs: Vec::new(),
            cache_read_errs: Vec::new(),
            command_execution_errs: Vec::new(),
            cache_write_errs: Vec::new(),
        };

        for err in err_results.into_iter().map(|res| res.unwrap_err()) {
            match err.kind() {
                work_skipping::InnerError::CommandCreation => {
                    error.command_creation_errs.push(err);
                }
                work_skipping::InnerError::CacheRead(_) => {
                    error.cache_read_errs.push(err);
                }
                work_skipping::InnerError::CommandExecution(_) => {
                    error.command_execution_errs.push(err);
                }
                work_skipping::InnerError::CacheWriteFailedButCommandExecutionSucceeded(_, _) => {
                    error.cache_write_errs.push(err);
                }
            };
        }

        error
    }

    pub fn into_result(self) -> Result<(), Self> {
        if self.has_any_non_cache_write_errors() || self.has_only_cache_write_errors() {
            Err(self)
        } else {
            Ok(())
        }
    }

    pub fn has_only_cache_write_errors(&self) -> bool {
        !self.has_any_non_cache_write_errors() && !self.cache_write_errs.is_empty()
    }

    fn has_any_non_cache_write_errors(&self) -> bool {
        !(self.command_creation_errs.is_empty()
            && self.bad_path_errs.is_empty()
            && self.cache_read_errs.is_empty()
            && self.command_execution_errs.is_empty())
    }
}

type Pair<'a> = (&'a str, &'a [work_skipping::Error<'a>]);
fn fmt_write(f: &mut fmt::Formatter<'_>, (msg, errs): &Pair) -> Option<fmt::Result> {
    if errs.is_empty() {
        None
    } else {
        Some(write!(f, "{}: {:#?}", msg, errs))
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errs_message_pairs: ReadonlyList<Pair> = Arc::new([
            (
                "Failed to create command:",
                self.command_creation_errs.as_slice(),
            ),
            ("Found bad path:", self.bad_path_errs.as_slice()),
            (
                "Fatal error when reading cache:",
                self.cache_read_errs.as_slice(),
            ),
            (
                "Command failed to execute:",
                self.command_execution_errs.as_slice(),
            ),
            (
                "Cache write failed after command execution success:",
                self.cache_write_errs.as_slice(),
            ),
        ]);

        errs_message_pairs
            .iter()
            .filter_map(|pair| fmt_write(f, pair))
            .collect::<fmt::Result>()
    }
}
