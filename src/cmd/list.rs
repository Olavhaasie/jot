use crate::config::Config;
use ansi_term::Colour::Purple;
use chrono::prelude::*;
use rusqlite::{Connection, Row, NO_PARAMS};
use std::error::Error;
use std::io;

fn print_entry(
    row: &Row,
    color: bool,
    json: bool,
    writer: &mut impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    let entry: String = row.get(0)?;
    let timestamp = row.get(1)?;
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
        .with_timezone(&Local)
        .format("%b %e %Y - %H:%M")
        .to_string();

    if json {
        writeln!(
            writer,
            "{{\"entry\":{:?},\"timestamp\":{}}}",
            entry, timestamp
        )?;
    } else if color {
        writeln!(writer, "{}\n{}", Purple.paint(format!("# {}", date)), entry)?;
    } else {
        writeln!(writer, "# {}\n{}", date, entry)?;
    }
    Ok(())
}

pub fn list(conn: &Connection, config: &Config) -> Result<(), Box<dyn Error>> {
    let from = config.from;
    let to = config.to;
    let pattern = &config.pattern;

    let mut query = String::from("SELECT entry, date FROM entries ");
    if from.is_some() || to.is_some() || pattern.is_some() {
        query.push_str("WHERE ");
    }
    let mut first = true;
    if let Some(f) = from {
        query.push_str(&format!("date > {} ", f));
        first = false;
    }
    if let Some(t) = to {
        if !first {
            query.push_str("AND ");
        }
        query.push_str(&format!("date < {} ", t));
        first = false;
    }
    if let Some(p) = pattern {
        if !first {
            query.push_str("AND ");
        }
        query.push_str(&format!("entry LIKE '%{}%' ", p));
    }

    if config.reverse {
        query.push_str("ORDER BY id DESC ");
    }

    if let Some(n) = config.count {
        query.push_str(&format!(
            "LIMIT {} OFFSET (SELECT COUNT(*) FROM entries) - {0} ",
            n
        ));
    }

    let mut statement = conn.prepare(&query)?;
    let mut rows = statement.query(NO_PARAMS)?;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = io::BufWriter::new(handle);
    let color = atty::is(atty::Stream::Stdout) && !config.no_color;
    while let Some(row) = rows.next()? {
        print_entry(&row, color, config.json, &mut writer)?;
    }

    Ok(())
}
