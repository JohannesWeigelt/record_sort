use core::fmt;
use std::io;

use serde::export::Formatter;

//Possible errors of a record-read-operation
#[derive(Debug)]
pub enum RecordIOError {
    IOError(io::Error),
    ParseJSONError(serde_json::error::Error),
}

impl std::error::Error for RecordIOError {}

impl std::fmt::Display for RecordIOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RecordIOError::IOError(e) => e.fmt(f),
            RecordIOError::ParseJSONError(e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for RecordIOError {
    fn from(error: io::Error) -> Self {
        RecordIOError::IOError(error)
    }
}

impl From<serde_json::error::Error> for RecordIOError {
    fn from(error: serde_json::error::Error) -> Self {
        RecordIOError::ParseJSONError(error)
    }
}