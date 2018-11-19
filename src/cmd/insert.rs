use clap::ArgMatches;
use sqlite::Connection;
use std::error::Error;
use std::io;
use std::io::{stdin, Read};
use std::process;

const INSERT_QUERY: &'static str =
    "INSERT INTO entries (entry, date) VALUES (?, strftime('%s','now'))";

fn get_entry_from_editor() -> Result<String, Box<Error>> {
    let mut tmp = tempfile::Builder::new()
        .prefix("jot-entry-")
        .suffix(".txt")
        .tempfile()?;

    let status = process::Command::new("vim")
        .arg(tmp.path().as_os_str())
        .status()?;

    if status.success() {
        let mut entry = String::new();
        tmp.read_to_string(&mut entry)?;
        tmp.close()?;
        Ok(entry)
    } else {
        tmp.close()?;
        Err(io::Error::new(
            io::ErrorKind::Other,
            "inserting entry aborted (editor exited with error code)",
        ).into())
    }
}

pub fn insert(connection: Connection, _matches: ArgMatches) -> Result<(), Box<Error>> {
    let mut statement = connection.prepare(INSERT_QUERY)?;

    let entry = get_entry_from_editor()?;

    eprintln!("Start typing:");
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input = input.into_bytes();

    statement.bind(1, &input[..])?;

    statement.next()?;
    Ok(())
}
