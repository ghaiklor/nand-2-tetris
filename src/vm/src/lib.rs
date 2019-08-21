pub mod config;
pub mod opcode;
pub mod parser;

use config::Config;
use std::fs;

pub fn run(config: Config) {
    let vm_code = fs::read_to_string(config.input_file).expect("Could not read the input file");
    let opcodes = parser::parse(&vm_code);

    println!("{:?}", opcodes);
}
