use rust_course::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing arguments!: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_course::run(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
