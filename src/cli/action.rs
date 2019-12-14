use core::fmt;
use std::convert::TryFrom;
use std::error::Error;

use serde::export::Formatter;

/// Possible actions to execute with record_sort
pub enum Action {
    Simple,
    Real,
    Fake,
    Generate,
}

impl TryFrom<Option<&String>> for Action {
    type Error = NoSuchActionError;

    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
        match value {
            None => Err(NoSuchActionError::new(String::new())),

            Some(arg) => match arg.as_str() {
                "simple" => Ok(Action::Simple),
                "real" => Ok(Action::Real),
                "fake" => Ok(Action::Real),
                "generate" => Ok(Action::Generate),
                _ => Err(NoSuchActionError::new(String::from(arg)))
            }
        }
    }
}

#[derive(Debug)]
pub struct NoSuchActionError {
    input: String
}

impl NoSuchActionError {
    pub fn new(input: String) -> Self {
        NoSuchActionError { input }
    }
}

impl fmt::Display for NoSuchActionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The input-string \"{}\" is no valid action.", &self.input)
    }
}

impl Error for NoSuchActionError {}

