use crate::cache::{ReadResult, WriteResult};
use crate::run_result::RunResult;
use common::RelativePath;

pub trait ReadCache {
    fn read(&self, path: &RelativePath) -> ReadResult<Option<RunResult>>;
}

pub trait WriteCache {
    fn write(&self, run_result: &RunResult) -> WriteResult<()>;
}
