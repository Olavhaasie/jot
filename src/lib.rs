extern crate chrono;
extern crate sqlite;

use chrono::prelude::*;
use chrono::Utc;
use config::{Command, Config};
use journal::Journal;
use sqlite::Connection;

use std::error::Error;
use std::io::Read;
use std::path::Path;

pub mod config;
mod journal;

const CREATE_QUERY: &'static str = "CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT, date INTEGER)";
const INSERT_QUERY: &'static str = "INSERT INTO entries (entry, date) VALUES (?, ?)";

pub fn run(config: Config) -> Result<(), Box<Error>> {
    match config.command {
        Command::Help => {
            println!("{}", config::HELP_INFO);
            Ok(())
        }
        Command::Version => {
            println!(
                "{} {}",
                config::NAME.unwrap_or(&config.name),
                config::VERSION.unwrap_or("unknown version")
            );
            Ok(())
        }
        Command::Edit => {
            println!("Start typing:");
            let stdin = std::io::stdin();
            let mut input = String::new();
            stdin.lock().read_to_string(&mut input)?;
            let input = input.into_bytes();

            let connection = get_connection(&config.filename)?;
            let mut statement = connection.prepare(INSERT_QUERY)?;

            statement.bind(1, &input[..])?;
            statement.bind(2, Local::now().timestamp())?;

            let mut cursor = statement.cursor();
            while let Some(row) = cursor.next().unwrap() {
                println!("{:?}", row);
            }
            Ok(())
        }
        Command::List => {
            let connection = get_connection(&config.filename)?;
            let statement = connection.prepare("SELECT * FROM entries")?;
            let mut cursor = statement.cursor();

            while let Some(row) = cursor.next().unwrap() {
                let timestamp = row[2].as_integer().unwrap();
                let date =
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
                println!("{}", timestamp);
                println!("# {}", date.with_timezone(&Local).format("%b %e %Y - %H:%M").to_string());
                println!(
                    "{}",
                    String::from_utf8(row[1].as_binary().unwrap().to_vec()).unwrap()
                );
            }
            Ok(())
        }
    }
}

fn get_connection(path: &str) -> Result<Connection, Box<Error>> {
    let file_exists = Path::new(path).is_file();
    let connection = sqlite::open(path)?;
    if !file_exists {
        connection.execute(CREATE_QUERY)?;
    }

    Ok(connection)
}
