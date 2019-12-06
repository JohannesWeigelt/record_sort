use std::env;

use crate::cli::console_application::ConsoleApplication;
use crate::data::reader::json_reader::JSONReader;
use crate::sort::merge_sort::MergeSort;

pub mod data;
pub mod util;
pub mod sort;
pub mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let limit = Some(args.get(2).unwrap().parse::<usize>().unwrap());
    let step = args.get(3).unwrap().parse::<usize>().unwrap();

    ConsoleApplication::new(JSONReader, MergeSort).run(path, limit, step);
}


