use compiler::config::Config;

fn main() {
    let config = Config::from_args();
    compiler::run(config);
}
