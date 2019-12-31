use core::fmt;
use std::error::Error;
use std::io;

use serde::export::Formatter;

use crate::data::record_io_error::RecordIOError;

// Possible errors of a benchmark-execution
#[derive(Debug)]
pub enum BenchmarkError {
    IOError(io::Error),
    RecordIOError(RecordIOError),
}

impl Error for BenchmarkError {}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BenchmarkError::IOError(e) => e.fmt(f),
            BenchmarkError::RecordIOError(e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for BenchmarkError {
    fn from(error: io::Error) -> Self {
        BenchmarkError::IOError(error)
    }
}

impl From<RecordIOError> for BenchmarkError {
    fn from(error: RecordIOError) -> Self {
        BenchmarkError::RecordIOError(error)
    }
}