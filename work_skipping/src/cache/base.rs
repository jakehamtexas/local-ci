use crate::cache::error::Result;
use crate::canonicalized_path::CanonicalizedPath;
use crate::run_result::RunResult;

pub trait Cache {
    fn write(&self, run_result: &RunResult) -> Result<()>;
    fn read(&self, path: &CanonicalizedPath) -> Result<Option<RunResult>>;
}
