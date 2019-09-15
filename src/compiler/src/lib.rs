pub mod config;

use config::Config;
use std::fs;

pub fn run(config: Config) {
    let source_code: String = fs::read_to_string(&config.input_file).expect("Can not read input file");

    println!("{:?}", source_code);
}
