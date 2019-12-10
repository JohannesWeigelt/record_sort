use std::str::FromStr;

/// Possible actions to execute with record_sort
pub enum Action {
    Simple,
    Real,
    Fake,
    Generate,
}

impl Action {
    pub fn get_key_word(&self) -> String {
        match self {
            Action::Simple => String::from("simple"),
            Action::Real => String::from("real"),
            Action::Fake => String::from("fake"),
            Action::Generate => String::from("generate"),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple" => Ok(Action::Simple),
            "real" => Ok(Action::Real),
            "fake" => Ok(Action::Real),
            "generate" => Ok(Action::Generate),
            _ => Err(String::from("The action is invalid."))
        }
    }
}