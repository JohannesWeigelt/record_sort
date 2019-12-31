use core::fmt;
use std::error::Error;
use std::fmt::Formatter;

use crate::benchmark::benchmark_error::BenchmarkError;
use crate::cli::action::NoSuchActionError;
use crate::data::record_io_error::RecordIOError;
use crate::sort::algorithm::NoSuchAlgorithmError;

#[derive(Debug)]
pub enum ApplicationError {
    NoSuchActionError(NoSuchActionError),
    NoSuchAlgorithmError(NoSuchAlgorithmError),
    BenchhmarkError(BenchmarkError),
    RecordIOError(RecordIOError)
}

impl Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::NoSuchActionError(e) => e.fmt(f),
            ApplicationError::NoSuchAlgorithmError(e) => e.fmt(f),
            ApplicationError::BenchhmarkError(e) => e.fmt(f),
            ApplicationError::RecordIOError(e) => e.fmt(f)
        }
    }
}

impl From<NoSuchActionError> for ApplicationError {
    fn from(error: NoSuchActionError) -> Self {
        ApplicationError::NoSuchActionError(error)
    }
}

impl From<NoSuchAlgorithmError> for ApplicationError {
    fn from(error: NoSuchAlgorithmError) -> Self {
        ApplicationError::NoSuchAlgorithmError(error)
    }
}

impl From<BenchmarkError> for ApplicationError {
    fn from(error: BenchmarkError) -> Self {
        ApplicationError::BenchhmarkError(error)
    }
}

impl From<RecordIOError> for ApplicationError {
    fn from(error: RecordIOError) -> Self {
        ApplicationError::RecordIOError(error)
    }
}

