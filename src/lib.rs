extern crate chrono;

use config::{Command, Config};
use journal::Journal;

use std::error::Error;
use std::io::Read;

pub mod config;
mod journal;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    match config.command {
        Command::Help => {
            println!("{}", config::HELP_INFO);
            Ok(())
        },
        Command::Version => {
            println!("{} {}", config::NAME.unwrap_or(&config.name), config::VERSION.unwrap_or("unknown version"));
            Ok(())
        },
        Command::Edit => {
            println!("Start typing:");
            let stdin = std::io::stdin();
            let mut input = String::new();
            stdin.lock().read_to_string(&mut input)?;

            Journal::write_entry(&input, &config.filename)
        },
        Command::List => {
            let journal = Journal::new(&config.filename)?;
            print!("{}", journal);
            Ok(())
        },
    }
}
