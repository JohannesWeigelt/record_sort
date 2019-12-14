use std::convert::TryFrom;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::metering_result::MeteringResult;
use crate::benchmark::real_data_benchmark::RealDataBenchmark;
use crate::benchmark::simple_benchmark::SimpleBenchmark;
use crate::cli::action::Action;
use crate::sort::algorithm::Algorithm;
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
        match Action::try_from(args.get(1)) {
            Err(no_such_action_error) => println!("{}", no_such_action_error),

            Ok(action) => match action {
                Action::Simple => self.simple(args),
                Action::Real => self.real(args),
                Action::Fake => unimplemented!(),
                Action::Generate => unimplemented!()
            }
        }
    }

    fn simple(&self, args: Vec<String>) {
        match Algorithm::try_from(args.get(2)) {
            Err(no_such_algorithm_error) => println!("{}", no_such_algorithm_error),

            Ok(algorithm) => {
                self.print_measurements(
                    SimpleBenchmark::new(RandomNumberGenerator).execute(self.sort_factory.create(algorithm))
                );
            }
        }
    }

    fn real(&self, args: Vec<String>) {
        match Algorithm::try_from(args.get(2)) {
            Err(no_such_algorithm_error) => println!("{}", no_such_algorithm_error),

            Ok(algorithm) => {
                self.print_measurements(
                    RealDataBenchmark::default().execute(self.sort_factory.create(algorithm))
                );
            }
        }
    }

    fn print_measurements(&self, measurements: Vec<MeteringResult>) {
        println!("Results: ");

        for result in measurements {
            println!("Lines: {}, Duration: {} seconds", result.get_sorted_elements(), result.get_duration())
        }
    }
}