use core::fmt;
use std::io;

use serde::export::Formatter;

//Possible errors of a record-read-operation
#[derive(Debug)]
pub enum RecordReadError {
    IOError(io::Error),
    ParseJSONError(serde_json::error::Error),
}

impl std::error::Error for RecordReadError {}

impl std::fmt::Display for RecordReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RecordReadError::IOError(e) => e.fmt(f),
            RecordReadError::ParseJSONError(e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for RecordReadError {
    fn from(error: io::Error) -> Self {
        RecordReadError::IOError(error)
    }
}

impl From<serde_json::error::Error> for RecordReadError {
    fn from(error: serde_json::error::Error) -> Self {
        RecordReadError::ParseJSONError(error)
    }
}