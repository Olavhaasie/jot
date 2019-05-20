use crate::config::Config;
use rusqlite::Connection;
use std::error::Error;

mod insert;
mod list;

use self::insert::insert;
use self::list::list;

pub enum Command {
    Insert,
    List,
}

impl Command {
    pub fn run(&self, conn: &Connection, config: &Config) -> Result<(), Box<Error>> {
        match self {
            Command::Insert => insert(conn, config),
            Command::List => list(conn, config),
        }
    }
}
