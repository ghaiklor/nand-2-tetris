use vm::config::Config;

fn main() {
    let config = Config::from_args();
    vm::run(config);
}
