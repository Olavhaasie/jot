extern crate chrono;
extern crate clap;
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
        }
        Command::List => {
            let from = config.matches.value_of("from").map(|f| {
                Local
                    .datetime_from_str(&format!("{} 00:00:00", f), "%d-%m-%Y %T")
                    .map(|d| d.timestamp())
            });
            let to = config.matches.value_of("to").map(|t| {
                Local
                    .datetime_from_str(&format!("{} 00:00:00", t), "%d-%m-%Y %T")
                    .map(|d| d.timestamp())
            });

            let mut query = String::from("SELECT * FROM entries");
            match (from, to) {
                (Some(f), Some(t)) => {
                    query.push_str(&format!(" WHERE date > {} AND date < {}", f?, t?))
                }
                (Some(f), None) => query.push_str(&format!(" WHERE date > {}", f?)),
                (None, Some(t)) => query.push_str(&format!(" WHERE date < {}", t?)),
                (None, None) => (),
            }
            let statement = connection.prepare(query)?;
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
