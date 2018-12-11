mod insert;
mod list;

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
    pub fn run(&self, conn: &Connection, matches: &ArgMatches) -> Result<(), Box<Error>> {
        match self {
            Command::Insert => insert(conn, matches),
            Command::List => list(conn, matches),
        }
    }
}
