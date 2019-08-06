pub mod codegen;
pub mod config;
pub mod parser;
pub mod symbols;

use codegen::CodeGen;
use config::Config;
use parser::Parser;
use std::fs;
use std::io::prelude::*;
use symbols::SymbolTable;

pub fn run(config: Config) {
    let assembly_code = fs::read_to_string(config.input_file).unwrap();

    let mut parser = Parser::new(&assembly_code);
    let instructions = parser.parse();

    let mut symbol_table = SymbolTable::new();
    symbol_table.resolve(instructions);

    let machine_code = CodeGen::gen(&instructions, &symbol_table);

    let mut out_file = fs::File::create(config.output_file).unwrap();
    out_file
        .write_all(machine_code.as_bytes())
        .expect("Could not write to the output file");
}
