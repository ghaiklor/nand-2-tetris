extern crate clap;
use clap::App;
use clap::Arg;

#[derive(Debug)]
pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(input_file: &str, output_file: &str) -> Config {
        let input_file = String::from(input_file);
        let output_file = String::from(output_file);

        Config {
            input_file,
            output_file,
        }
    }

    pub fn from_args() -> Config {
        let matches = App::new("hasm")
            .version("0.0.0")
            .author("Eugene Obrezkov <ghaiklor@gmail.com>")
            .about("Assembler for the Hack CPU")
            .arg(
                Arg::with_name("input_file")
                    .short("i")
                    .long("input")
                    .value_name("INPUT_FILE")
                    .help("Set an input file of assembly code")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("output_file")
                    .short("o")
                    .long("output")
                    .value_name("OUTPUT_FILE")
                    .help("Set an output file of machine instructions")
                    .takes_value(true)
                    .required(false)
                    .default_value("output.hack"),
            )
            .get_matches();

        let input_file = matches.value_of("input_file").unwrap();
        let output_file = matches.value_of("output_file").unwrap_or_default();

        Config::new(input_file, output_file)
    }
}
