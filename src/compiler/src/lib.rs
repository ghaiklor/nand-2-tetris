pub mod config;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod token;

use config::Config;
use parser::Parser;
use scanner::Scanner;
use std::fs;

pub fn run(config: Config) {
    let source_code = fs::read_to_string(&config.input_file).expect("Can not read input file");

    let tokens = Scanner::new(&source_code).scan();
    if let Some(path) = config.tokens_file {
        printer::print_tokens(&tokens, &path);
    }

    let ast = Parser::new(&tokens).parse();
    if let Some(path) = config.ast_file {
        printer::print_ast(&ast, &path);
    }
}
