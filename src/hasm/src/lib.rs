pub mod config;

use config::Config;
use std::fs;

pub fn run(config: Config) {
    let assembly_code = fs::read_to_string(config.input_file).unwrap();
    println!("{}", assembly_code);
}
