use chrono::{prelude::*, ParseResult};
use clap::ArgMatches;
use rusqlite::{Connection, Row, NO_PARAMS};
use std::error::Error;

fn parse_date(s: &str) -> ParseResult<i64> {
    Local
        .datetime_from_str(&format!("{} 00:00:00", s), "%d-%m-%Y %T")
        .map(|d| d.timestamp())
}

fn print_entry(row: &Row, color: bool, json: bool) {
    let entry: String = row.get(0);
    let timestamp = row.get(1);
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
        .with_timezone(&Local)
        .format("%b %e %Y - %H:%M")
        .to_string();

    if json {
        println!("{{\"entry\":{:?},\"timestamp\":{}}}", entry, timestamp);
    } else {
        println!(
            "{}# {}{}\n{}",
            if color { "\x1b[1;35m" } else { "" },
            date,
            if color { "\x1b[0m" } else { "" },
            entry,
        );
    }
}

pub fn list(conn: &Connection, matches: &ArgMatches) -> Result<(), Box<Error>> {
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

    if matches.is_present("reverse") {
        query.push_str("ORDER BY id DESC ");
    }

    if let Some(n) = matches.value_of("count") {
        query.push_str(&format!(
            "LIMIT {} OFFSET (SELECT COUNT(*) FROM entries) - {0} ",
            n
        ));
    }

    let mut statement = conn.prepare(&query)?;
    let mut rows = statement.query(NO_PARAMS)?;

    while let Some(result_row) = rows.next() {
        let row = result_row?;
        let color = atty::is(atty::Stream::Stdout) && !matches.is_present("nocolor");
        print_entry(&row, color, matches.is_present("json"));
    }

    Ok(())
}
