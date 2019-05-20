#[macro_use]
extern crate clap;

mod cmd;
pub mod config;

use self::config::Config;
use ansi_term::Colour::Yellow;
use rusqlite::Connection;

use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

const DEFAULT_FILENAME: &str = "journal.sqlite";

const CREATE_QUERY: &str = "BEGIN;
    CREATE TABLE entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT NOT NULL, date INTEGER NOT NULL);
    CREATE INDEX date_index on entries(date);
    COMMIT;";

pub fn run(config: &Config) -> Result<(), Box<Error>> {
    let path = config
        .matches
        .value_of("db")
        .map(PathBuf::from)
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(DEFAULT_FILENAME));

    let (connection, created) = get_connection(&path)?;

    if created && atty::is(atty::Stream::Stdout) {
        let color = !config.matches.is_present("nocolor");
        let msg = format!("Created new journal database {:?}", path);
        if color {
            println!("{}", Yellow.bold().paint(msg));
        } else {
            println!("{}", msg);
        }
    }

    config.command.run(&connection, &config.matches)
}

fn get_connection(path: &Path) -> Result<(Connection, bool), Box<Error>> {
    let path_exists = path.exists();
    if path_exists && !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "given path is not a file").into());
    }

    let connection = Connection::open(path)?;
    if !path_exists {
        connection.execute_batch(CREATE_QUERY)?;
    }

    Ok((connection, !path_exists))
}
