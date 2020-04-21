use crate::config::Config;
use rusqlite::Connection;
use std::{error::Error, io, io::Read, process};

const INSERT_QUERY: &str = "INSERT INTO entries (entry, date) VALUES (?, strftime('%s','now'))";

fn get_entry_from_editor(editor: &str) -> Result<String, Box<dyn Error>> {
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
        )
        .into())
    }
}

fn get_entry(config: &Config) -> Result<String, Box<dyn Error>> {
    if let Some(editor) = &config.editor {
        get_entry_from_editor(&editor)
    } else {
        if atty::is(atty::Stream::Stdin) {
            println!("Press ^D to save and ^C to abort.");
            println!("Start typing:");
        }
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(input)
    }
}

pub fn insert(conn: &Connection, config: &Config) -> Result<(), Box<dyn Error>> {
    let entry = get_entry(&config)?;

    if !entry.is_empty() {
        let mut statement = conn.prepare(INSERT_QUERY)?;
        statement.execute(&[entry])?;
    }
    Ok(())
}
