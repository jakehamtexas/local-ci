use crate::cache::{ReadResult, WriteResult};
use crate::run_result::RunResult;
use common::canonicalized_path::CanonicalizedPath;

pub trait ReadCache {
    fn read(&self, path: &CanonicalizedPath) -> ReadResult<Option<RunResult>>;
}

pub trait WriteCache {
    fn write(&self, run_result: &RunResult) -> WriteResult<()>;
}
