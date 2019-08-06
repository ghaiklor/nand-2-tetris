pub mod config;
pub mod parser;
pub mod symbols;

use config::Config;
use parser::Parser;
use std::fs;
use symbols::SymbolTable;

pub fn run(config: Config) {
    let assembly_code = fs::read_to_string(config.input_file).unwrap();
    let mut parser = Parser::new(&assembly_code);
    let instructions = parser.parse();
    let mut symbol_table = SymbolTable::new();
    symbol_table.resolve(instructions);

    println!("{:?}", symbol_table);
}
