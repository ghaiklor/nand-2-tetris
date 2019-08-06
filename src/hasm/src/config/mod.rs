use clap;

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(args: &clap::ArgMatches) -> Config {
        let input_file = String::from(args.value_of("input_file").unwrap());
        let output_file = String::from(args.value_of("output_file").unwrap());

        Config {
            input_file,
            output_file,
        }
    }
}
