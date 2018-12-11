use chrono::prelude::*;
use chrono::ParseResult;
use clap::ArgMatches;
use rusqlite::{Connection, Row, NO_PARAMS};
use std::error::Error;

use super::super::config::Order;

fn parse_date(s: &str) -> ParseResult<i64> {
    Local
        .datetime_from_str(&format!("{} 00:00:00", s), "%d-%m-%Y %T")
        .map(|d| d.timestamp())
}

fn print_entry(row: &Row, color: bool) {
    let entry: String = row.get(0);
    let date: DateTime<Local> = row.get(1);

    println!(
        "{}# {}{}\n{}",
        if color { "\x1b[1;35m" } else { "" },
        date.format("%b %e %Y - %H:%M").to_string(),
        if color { "\x1b[0m" } else { "" },
        entry,
    );
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

    query.push_str("ORDER BY id ");
    query.push_str(match value_t!(matches, "sort", Order).unwrap() {
        Order::Asc => "ASC ",
        Order::Desc => "DESC ",
    });

    if let Some(n) = matches.value_of("count") {
        query.push_str(&format!(
            "LIMIT {} OFFSET (SELECT COUNT(*) FROM entries) - {0} ",
            n
        ));
    }

    println!("{}", query);
    let mut statement = conn.prepare(&query)?;
    let mut rows = statement.query(NO_PARAMS)?;

    while let Some(result_row) = rows.next() {
        let row = result_row?;
        print_entry(&row, !matches.is_present("nocolor"));
    }

    Ok(())
}
