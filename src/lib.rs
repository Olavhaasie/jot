extern crate chrono;

use chrono::prelude::*;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};

mod journal;
use journal::{Entry, Journal};

const DEFAULT_FILENAME: &str = "journal.txt";

pub enum Command {
    Help,
    Version,
    Edit,
    List,
}

pub struct Config {
    pub name: String,
    pub command: Command,
    pub filename: String,
    pub color: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let name = args.next().unwrap();

        let command = match args.next() {
            Some(ref s) if s == "ls" => Command::List,
            Some(_) => return Err("Unknown command"),
            None => Command::Edit,
        };

        Ok(Config { name, command, filename: DEFAULT_FILENAME.to_string(), color: true })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    match config.command {
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

