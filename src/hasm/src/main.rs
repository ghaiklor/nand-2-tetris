extern crate clap;

use clap::App;
use clap::Arg;
use hasm::config::Config;

fn main() {
    let matches = App::new("hasm")
        .version("0.0.0")
        .author("Eugene Obrezkov <ghaiklor@gmail.com>")
        .about("An assembler for the Hack Assembly Language")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input")
                .value_name("INPUT_FILE")
                .help("Provide an input file where assembly code persists")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Provide an output file where generated machine code will persists")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config = Config::new(&matches);
    hasm::run(config);
}
