pub mod codegen;
pub mod config;
pub mod instructions;
pub mod parser;
pub mod symbols;

use config::Config;
use std::fs;
use std::io::prelude::*;

pub fn run(config: Config) {
    let assembly_code = fs::read_to_string(config.input_file).expect("Could not read input file");
    let instructions = parser::parse(&assembly_code);
    let symbol_table = symbols::resolve(&instructions);
    let machine_code = codegen::codegen(&instructions, &symbol_table);

    let mut out_file = fs::File::create(config.output_file).expect("Could not open output file");
    out_file
        .write_all(machine_code.as_bytes())
        .expect("Could not write to the output file");
}
