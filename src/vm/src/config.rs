pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(input_file: &str, output_file: &str) -> Config {
        Config {
            input_file: String::from(input_file),
            output_file: String::from(output_file),
        }
    }

    pub fn from_args() -> Config {
        let matches = clap::App::new("vm")
            .version(clap::crate_version!())
            .author("Eugene Obrezkov <ghaiklor@gmail.com>")
            .about("Translator from stack virtual machine code to Hack Assembly")
            .arg_from_usage("-i --input=<INPUT-FILE> 'Set an input file where vm code persists'")
            .arg_from_usage("-o --output=<OUTPUT-FILE> 'Set an output file where translated code will be stored'")
            .get_matches();

        let input_file = matches
            .value_of("input")
            .expect("Missing --input parameter");

        let output_file = matches
            .value_of("output")
            .expect("Missing --output parameter");

        Config::new(input_file, output_file)
    }
}
