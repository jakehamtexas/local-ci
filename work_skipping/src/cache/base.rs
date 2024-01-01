use crate::cache::error::Result;
use crate::run_result::RunResult;
use std::path::Path;

pub trait Cache {
    fn write(&self, run_result: &RunResult) -> Result<()>;
    fn read(&self, path: &Path) -> Result<Option<RunResult>>;
}
