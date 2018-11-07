extern crate clap;
extern crate chrono;
extern crate chrono_english;
extern crate sqlite;

use chrono::prelude::*;
use chrono::Utc;
use config::{Command, Config};
use sqlite::Connection;

use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};

pub mod config;

const CREATE_QUERY: &'static str =
    "CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT, date INTEGER)";
const INSERT_QUERY: &'static str = "INSERT INTO entries (entry, date) VALUES (?, ?)";

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = PathBuf::from(config.matches.value_of("db").unwrap());
    match config.command {
        Command::Edit => {
            let connection = get_connection(&path)?;
            let mut statement = connection.prepare(INSERT_QUERY)?;

            println!("Start typing:");
            let stdin = std::io::stdin();
            let mut input = String::new();
            stdin.lock().read_to_string(&mut input)?;
            let input = input.into_bytes();

            statement.bind(1, &input[..])?;
            statement.bind(2, Local::now().timestamp())?;

            statement.next()?;
        }
        Command::List => {
            let connection = get_connection(&path)?;
            let statement = connection.prepare("SELECT * FROM entries")?;
            let mut cursor = statement.cursor();

            while let Some(row) = cursor.next().unwrap() {
                let timestamp = row[2].as_integer().unwrap();
                let date =
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
                println!(
                    "\x1b[1;35m# {}\x1b[0m",
                    date.with_timezone(&Local)
                        .format("%b %e %Y - %H:%M")
                        .to_string()
                );
                println!(
                    "{}",
                    String::from_utf8(row[1].as_binary().unwrap().to_vec()).unwrap()
                );
            }
        }
    };
    Ok(())
}

fn get_connection(path: &Path) -> Result<Connection, Box<Error>> {
    let file_exists = path.exists();
    let connection = sqlite::open(path.to_str().unwrap())?;
    if !file_exists {
        connection.execute(CREATE_QUERY)?;
    }

    Ok(connection)
}
