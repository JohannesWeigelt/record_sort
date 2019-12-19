use core::fmt;
use std::error::Error;
use std::io;

use serde::export::Formatter;

use crate::data::reader::record_read_error::RecordReadError;

// Possible errors of a benchmark-execution
#[derive(Debug)]
pub enum BenchmarkError {
    RecordReadError(RecordReadError),
    IOError(io::Error),
}

impl Error for BenchmarkError {}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BenchmarkError::RecordReadError(e) => e.fmt(f),
            BenchmarkError::IOError(e) => e.fmt(f)
        }
    }
}

impl From<RecordReadError> for BenchmarkError {
    fn from(error: RecordReadError) -> Self {
        BenchmarkError::RecordReadError(error)
    }
}

impl From<io::Error> for BenchmarkError {
    fn from(error: io::Error) -> Self {
        BenchmarkError::IOError(error)
    }
}