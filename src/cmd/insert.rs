use clap::ArgMatches;
use rusqlite::Connection;
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
        if atty::is(atty::Stream::Stdin) {
            println!("Press ^D to save and ^C to abort.");
            println!("Start typing:");
        }
        let mut input = String::new();
        stdin().read_to_string(&mut input)?;
        Ok(input)
    }
}

pub fn insert(connection: Connection, matches: ArgMatches) -> Result<(), Box<Error>> {
    let entry = get_entry(&matches)?;

    if !entry.is_empty() {
        let mut statement = connection.prepare(INSERT_QUERY)?;
        statement.execute(&[entry])?;
    }
    Ok(())
}
