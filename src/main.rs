extern crate jot;

use jot::config::Config;
use std::error::Error;

fn main() {
    if let Err(e) = error_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn error_main() -> Result<(), Box<Error>> {
    let config = Config::new();
    jot::run(config)
}
