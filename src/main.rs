use std::env;

use crate::cli::console_application::ConsoleApplication;
use crate::sort::sort_factory::SortFactory;
use crate::data::writer::json_writer::JSONWriter;
use crate::data::generation::review_generator::ReviewGenerator;
use crate::data::writer::record_writer::RecordWriter;

pub mod data;
pub mod util;
pub mod sort;
pub mod benchmark;
pub mod cli;

fn main() {
//    ConsoleApplication::new(SortFactory).run_record_sort(env::args().collect());
    let writer = JSONWriter;
    writer.write(&String::from("data_sets/gen.json"), &mut ReviewGenerator::new(&mut rand::thread_rng()), 5);
}


