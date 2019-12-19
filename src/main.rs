use std::env;

use crate::cli::console_application::ConsoleApplication;
use crate::sort::sort_factory::SortFactory;

pub mod data;
pub mod util;
pub mod sort;
pub mod benchmark;
pub mod cli;

fn main() {
    ConsoleApplication::new(SortFactory).run_record_sort(env::args().collect());
}


