use clap::ArgMatches;
use sqlite::Connection;
use std::error::Error;
use std::io;
use std::io::{stdin, Read};
use std::process;

const INSERT_QUERY: &'static str =
    "INSERT INTO entries (entry, date) VALUES (?, strftime('%s','now'))";

fn get_entry_from_editor(editor: &str) -> Result<String, Box<Error>> {
    let mut tmp = tempfile::Builder::new()
        .prefix("jot-entry-")
        .suffix(".txt")
        .tempfile()?;

    let status = process::Command::new(editor)
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

fn get_entry(matches: &ArgMatches) -> Result<String, Box<Error>> {
    if let Some(editor) = matches.value_of("editor") {
        get_entry_from_editor(editor)
    } else {
        eprintln!("Start typing:");
        let mut input = String::new();
        stdin().read_to_string(&mut input)?;
        Ok(input)
    }
}

pub fn insert(connection: Connection, matches: ArgMatches) -> Result<(), Box<Error>> {
    let mut statement = connection.prepare(INSERT_QUERY)?;

    let entry = get_entry(&matches)?;

    statement.bind(1, entry.as_bytes())?;

    statement.next()?;
    Ok(())
}
