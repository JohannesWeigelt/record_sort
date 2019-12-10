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