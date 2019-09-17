pub mod config;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod token;

use config::Config;
use parser::Parser;
use scanner::Scanner;
use std::fs;
use std::path::Path;

pub fn run(config: Config) {
    let input_files: Vec<_> = if fs::metadata(&config.input_file).unwrap().is_dir() {
        fs::read_dir(&config.input_file)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "jack")
            .map(|path| path.to_str().unwrap().to_string())
            .collect()
    } else {
        vec![String::from(&config.input_file)]
    };

    for input_file in input_files {
        let source_code = fs::read_to_string(&input_file).expect("Can not read input file");
        let tokens = Scanner::new(&source_code).scan();
        if config.emit_tokens {
            let path = Path::new(&input_file)
                .with_extension("tok")
                .to_str()
                .unwrap()
                .to_owned();
            printer::print_tokens(&tokens, &path);
        }

        let ast = Parser::new(&tokens).parse();
        if config.emit_ast {
            let path = Path::new(&input_file)
                .with_extension("ast")
                .to_str()
                .unwrap()
                .to_owned();
            printer::print_ast(&ast, &path);
        }
    }
}
