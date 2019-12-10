use std::str::FromStr;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::metering_result::MeteringResult;
use crate::benchmark::real_data_benchmark::RealDataBenchmark;
use crate::benchmark::simple_benchmark::SimpleBenchmark;
use crate::cli::action::Action;
use crate::data::reader::json_reader::JSONReader;
use crate::sort::sort_factory::SortFactory;
use crate::util::random_number_generator::RandomNumberGenerator;

pub struct ConsoleApplication {
    sort_factory: SortFactory,
}

impl ConsoleApplication {
    pub fn new(sort_factory: SortFactory) -> Self {
        ConsoleApplication {
            sort_factory,
        }
    }

    pub fn run(&self, args: Vec<String>) {
        let action = Action::from_str(args.get(1).unwrap()).unwrap();

        match action {
            Action::Simple => {
                self.print_measurements(
                    SimpleBenchmark::new(RandomNumberGenerator).execute(
                        self.sort_factory.create(
                            args.get(2).unwrap()
                        ).unwrap()
                    )
                );
            }

            Action::Real => {
                self.print_measurements(
                    RealDataBenchmark::new(
                        JSONReader,
                        String::from("data_sets/foo_bar.json"),
                        Some(10000000),
                        100000
                    ).execute(
                        self.sort_factory.create(
                            args.get(2).unwrap()
                        ).unwrap()
                    )
                );
            },
            Action::Fake => unimplemented!(),
            Action::Generate => unimplemented!()
        }
    }

    fn print_measurements(&self, measurements: Vec<MeteringResult>) {
        println!("Results: ");

        for result in measurements {
            println!("Lines: {}, Duration: {} seconds", result.get_sorted_elements(), result.get_duration())
        }
    }
}