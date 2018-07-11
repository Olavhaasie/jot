extern crate chrono;

use chrono::prelude::*;

use config::{Command, Config};
use journal::Journal;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};

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

            write_entry(&input, &config.filename)
        },
        Command::List => {
            let journal = Journal::new(&config.filename)?;
            print!("{}", journal);
            Ok(())
        },
    }
}

fn write_entry(entry: &str, filename: &str) -> Result<(), Box<Error>> {
    let journal = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    let mut journal = BufWriter::new(journal);

    journal.write(b"# ")?;
    journal.write(Local::now().to_rfc3339().as_bytes())?;
    journal.write(b"\n")?;
    journal.write(entry.as_bytes())?;
    journal.write(b"\n")?;
    journal.flush()?;

    Ok(())
}

