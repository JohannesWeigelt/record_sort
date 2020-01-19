use core::fmt;
use std::convert::TryFrom;
use std::fmt::Formatter;

use crate::sort::algorithm::Algorithm::{Merge, Quick, Insertion};

pub enum Algorithm {
    Merge,
    Quick,
    Insertion
}

impl TryFrom<Option<&String>> for Algorithm {
    type Error = NoSuchAlgorithmError;

    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
        match value {
            None => Err(NoSuchAlgorithmError::new(String::new())),

            Some(arg) => match arg.as_str() {
                "merge" => Ok(Merge),
                "quick" => Ok(Quick),
                "insertion" => Ok(Insertion),
                _ => Err(NoSuchAlgorithmError::new(String::from(arg)))
            }
        }
    }
}

#[derive(Debug)]
pub struct NoSuchAlgorithmError {
    input: String
}

impl NoSuchAlgorithmError {
    pub fn new(input: String) -> Self {
        NoSuchAlgorithmError { input }
    }
}

impl fmt::Display for NoSuchAlgorithmError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The input-string \"{}\" is no valid sort-algorithm.", &self.input)
    }
}