extern crate chrono;
extern crate clap;
extern crate sqlite;

mod cmd;
pub mod config;

use config::Config;
use sqlite::Connection;

use std::error::Error;
use std::path::{Path, PathBuf};

const CREATE_QUERY: &'static str =
    "CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT, date INTEGER)";

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = PathBuf::from(config.matches.value_of("db").unwrap());
    let connection = get_connection(&path)?;

    config.command.run(connection, config.matches)
}

fn get_connection(path: &Path) -> Result<Connection, Box<Error>> {
    let file_exists = path.exists();
    let connection = sqlite::open(path.to_str().unwrap())?;
    if !file_exists {
        connection.execute(CREATE_QUERY)?;
    }

    Ok(connection)
}
