mod config;
mod parser;

use config::Config;
use std::fs;

pub fn run() {
    let config = Config::from_args();
    let input_file = config.input_file;
    let output_file = config.output_file;

    let assembly_code = fs::read_to_string(input_file).unwrap();
    println!("{}", assembly_code);
}
