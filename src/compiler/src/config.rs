pub struct Config {
    pub input_file: String,
    pub emit_tokens: bool,
    pub emit_ast: bool,
}

impl Config {
    pub fn new(input_file: &str, emit_tokens: bool, emit_ast: bool) -> Config {
        Config {
            input_file: String::from(input_file),
            emit_tokens,
            emit_ast,
        }
    }

    pub fn from_args() -> Config {
        let matches = clap::App::new("compiler")
            .version(clap::crate_version!())
            .author("Eugene Obrezkov <ghaiklor@gmail.com>")
            .about("Compiler for the Jack language into stack VM code")
            .arg_from_usage("-i --input=<INPUT-FILE> 'Set an input file where Jack code persists'")
            .arg_from_usage("--emit-tokens 'Emit tokens sequence into debug file'")
            .arg_from_usage("--emit-ast 'Emit parsed tree into XML file")
            .get_matches();

        let input_file = matches
            .value_of("input")
            .expect("Missing --input parameter");

        let emit_tokens = matches.is_present("emit-tokens");
        let emit_ast = matches.is_present("emit-ast");

        Config::new(input_file, emit_tokens, emit_ast)
    }
}
