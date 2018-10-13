extern crate jot;

use jot::config::Config;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = jot::run(config) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
