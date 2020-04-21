use jot::config::Config;
use std::error::Error;
use structopt::StructOpt;

fn main() {
    if let Err(e) = error_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn error_main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    jot::run(&config)
}
