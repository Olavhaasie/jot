extern crate chrono;
extern crate clap;
extern crate sqlite;

pub mod config;
mod cmd;

use chrono::prelude::*;
use config::{Command, Config};
use sqlite::Connection;

use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};


const CREATE_QUERY: &'static str =
    "CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT, date INTEGER)";
const INSERT_QUERY: &'static str = "INSERT INTO entries (entry, date) VALUES (?, ?)";

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = PathBuf::from(config.matches.value_of("db").unwrap());
    let connection = get_connection(&path)?;

    match config.command {
        Command::Edit => {
            let mut statement = connection.prepare(INSERT_QUERY)?;

            println!("Start typing:");
            let stdin = std::io::stdin();
            let mut input = String::new();
            stdin.lock().read_to_string(&mut input)?;
            let input = input.into_bytes();

            statement.bind(1, &input[..])?;
            statement.bind(2, Local::now().timestamp())?;

            statement.next()?;
            Ok(())
        }
        Command::List => cmd::list(connection, config.matches),
    }
}

fn get_connection(path: &Path) -> Result<Connection, Box<Error>> {
    let file_exists = path.exists();
    let connection = sqlite::open(path.to_str().unwrap())?;
    if !file_exists {
        connection.execute(CREATE_QUERY)?;
    }

    Ok(connection)
}
