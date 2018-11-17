use chrono::prelude::*;
use clap::ArgMatches;
use sqlite::Connection;
use std::error::Error;
use std::io::Read;

const INSERT_QUERY: &'static str = "INSERT INTO entries (entry, date) VALUES (?, ?)";

pub fn insert(connection: Connection, _matches: ArgMatches) -> Result<(), Box<Error>> {
    let mut statement = connection.prepare(INSERT_QUERY)?;

    println!("Start typing:");
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.into_bytes();

    statement.bind(1, &input[..])?;
    statement.bind(2, Local::now().timestamp())?;

    statement.next()?;
    Ok(())
}
