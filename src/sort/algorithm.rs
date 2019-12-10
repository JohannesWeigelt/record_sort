use std::str::FromStr;
use crate::sort::algorithm::Algorithm::Merge;

pub enum Algorithm {
    Merge
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merge" => Ok(Merge),
            _ => Err(String::from("Invalid sorting algorithm"))
        }
    }
}