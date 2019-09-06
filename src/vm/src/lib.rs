pub mod codegen;
pub mod config;
pub mod opcode;
pub mod parser;

use codegen::Codegen;
use config::Config;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn run(config: Config) {
    let input_files: Vec<_> = if fs::metadata(&config.input_file).unwrap().is_dir() {
        fs::read_dir(&config.input_file)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "vm")
            .map(|path| path.to_str().unwrap().to_string())
            .collect()
    } else {
        vec![String::from(&config.input_file)]
    };

    let mut output_file = File::create(&config.output_file).expect("Can not create an output file");
    if fs::metadata(&config.input_file).unwrap().is_dir() {
        output_file
            .write_all(Codegen::new(&config.output_file).emit_entry().as_bytes())
            .expect("Could not write entry in assembly file");
    }

    for input_file in input_files {
        let vm_code = fs::read_to_string(&input_file).expect("Could not read the input file");
        let opcodes = parser::parse(&vm_code);
        let assembly = Codegen::new(&input_file).codegen(&opcodes);

        output_file
            .write_all(assembly.as_bytes())
            .expect("Could not write to the output file");
    }
}
