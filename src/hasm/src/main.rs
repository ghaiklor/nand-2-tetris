use hasm::config::Config;

fn main() {
    let config = Config::from_args();
    hasm::run(config);
}
