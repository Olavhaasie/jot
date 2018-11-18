use clap::ArgMatches;
use sqlite::Connection;
use std::error::Error;
use std::io::{stdin, Read};

const INSERT_QUERY: &'static str =
    "INSERT INTO entries (entry, date) VALUES (?, strftime('%s','now'))";

pub fn insert(connection: Connection, _matches: ArgMatches) -> Result<(), Box<Error>> {
    let mut statement = connection.prepare(INSERT_QUERY)?;

    eprintln!("Start typing:");
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input = input.into_bytes();

    statement.bind(1, &input[..])?;

    statement.next()?;
    Ok(())
}
