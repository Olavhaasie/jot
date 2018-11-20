extern crate atty;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate sqlite;
extern crate tempfile;

mod cmd;
pub mod config;

use config::Config;
use sqlite::Connection;

use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

const CREATE_QUERY: &'static str =
    "CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT NOT NULL, date INTEGER NOT NULL);\
        CREATE INDEX date_index on entries(date);";

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = PathBuf::from(config.matches.value_of("db").unwrap());
    let connection = get_connection(&path)?;

    config.command.run(connection, config.matches)
}

fn get_connection(path: &Path) -> Result<Connection, Box<Error>> {
    let path_exists = path.exists();
    if path_exists && !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "given path is not a file").into());
    }

    let connection = sqlite::open(path.to_str().unwrap())?;
    if !path_exists {
        connection.execute(CREATE_QUERY)?;
    }

    Ok(connection)
}
