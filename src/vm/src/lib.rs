pub mod codegen;
pub mod config;
pub mod opcode;
pub mod parser;

use codegen::Codegen;
use config::Config;
use std::fs;

pub fn run(config: Config) {
    let vm_code = fs::read_to_string(&config.input_file).expect("Could not read the input file");
    let opcodes = parser::parse(&vm_code);
    let assembly = Codegen::new(&config.input_file).codegen(&opcodes);

    fs::write(config.output_file, assembly).expect("Could not write to the output file");
}
