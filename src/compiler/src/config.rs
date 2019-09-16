pub struct Config {
    pub input_file: String,
    pub output_file: String,
    pub tokens_file: Option<String>,
    pub ast_file: Option<String>,
}

impl Config {
    pub fn new(
        input_file: &str,
        output_file: &str,
        tokens_file: Option<String>,
        ast_file: Option<String>,
    ) -> Config {
        Config {
            input_file: String::from(input_file),
            output_file: String::from(output_file),
            tokens_file,
            ast_file,
        }
    }

    pub fn from_args() -> Config {
        let matches = clap::App::new("compiler")
            .version(clap::crate_version!())
            .author("Eugene Obrezkov <ghaiklor@gmail.com>")
            .about("Compiler for the Jack language into stack VM code")
            .arg_from_usage("-i --input=<INPUT-FILE> 'Set an input file where Jack code persists'")
            .arg_from_usage("-o --output=<OUTPUT-FILE> 'Set an output file where translated code will be stored'")
            .arg_from_usage("--emit-tokens=[TOKENS-FILE] 'Set an output file where tokens will be stored'")
            .arg_from_usage("--emit-ast=[AST-FILE] 'Set an output file where AST will be stored'")
            .get_matches();

        let input_file = matches
            .value_of("input")
            .expect("Missing --input parameter");

        let output_file = matches
            .value_of("output")
            .expect("Missing --output parameter");

        let tokens_file = match matches.value_of("emit-tokens") {
            Some(path) => Option::Some(String::from(path)),
            None => Option::None,
        };

        let ast_file = match matches.value_of("emit-ast") {
            Some(path) => Option::Some(String::from(path)),
            None => Option::None,
        };

        Config::new(input_file, output_file, tokens_file, ast_file)
    }
}
