use chrono::prelude::*;
use chrono::ParseResult;
use clap::ArgMatches;
use sqlite::{Connection, Value};
use std::error::Error;

fn parse_date(s: &str) -> ParseResult<i64> {
    Local
        .datetime_from_str(&format!("{} 00:00:00", s), "%d-%m-%Y %T")
        .map(|d| d.timestamp())
}

fn print_entry(row: &[Value], color: bool) {
    let timestamp = row[1].as_integer().unwrap();
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    println!(
        "{}# {}{}\n{}",
        if color { "\x1b[1;35m" } else { "" },
        date.with_timezone(&Local)
            .format("%b %e %Y - %H:%M")
            .to_string(),
        if color { "\x1b[0m" } else { "" },
        String::from_utf8(row[0].as_binary().unwrap().to_vec()).unwrap(),
    );
}

pub fn list(connection: Connection, matches: ArgMatches) -> Result<(), Box<Error>> {
    let from = matches.value_of("from").map(|f| parse_date(f));
    let to = matches.value_of("to").map(|t| parse_date(t));
    let pattern = matches.value_of("pattern");

    let mut query = String::from("SELECT entry, date FROM entries ");
    if from.is_some() || to.is_some() || pattern.is_some() {
        query.push_str("WHERE ");
    }
    let mut first = true;
    if let Some(f) = from {
        query.push_str(&format!("date > {} ", f?));
        first = false;
    }
    if let Some(t) = to {
        if !first {
            query.push_str("AND ");
        }
        query.push_str(&format!("date < {} ", t?));
        first = false;
    }
    if let Some(p) = pattern {
        if !first {
            query.push_str("AND ");
        }
        query.push_str(&format!("entry LIKE '%{}%' ", p));
    }

    let statement = connection.prepare(query)?;
    let mut cursor = statement.cursor();

    while let Some(row) = cursor.next()? {
        print_entry(row, !matches.is_present("nocolor"));
    }

    Ok(())
}
