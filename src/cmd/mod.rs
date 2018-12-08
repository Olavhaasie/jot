pub mod insert;
pub mod list;

use self::insert::insert;
use self::list::list;

use clap::ArgMatches;
use rusqlite::Connection;
use std::error::Error;

pub enum Command {
    Insert,
    List,
}

impl Command {
    pub fn run(&self, connection: Connection, matches: ArgMatches) -> Result<(), Box<Error>> {
        match self {
            Command::Insert => insert(connection, matches),
            Command::List => list(connection, matches),
        }
    }
}
