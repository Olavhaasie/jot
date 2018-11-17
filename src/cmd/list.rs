use chrono::prelude::*;
use clap::ArgMatches;
use std::error::Error;
use sqlite::Connection;

pub fn list(connection: Connection, matches: ArgMatches) -> Result<(), Box<Error>> {
    let from = matches.value_of("from").map(|f| {
        Local
            .datetime_from_str(&format!("{} 00:00:00", f), "%d-%m-%Y %T")
            .map(|d| d.timestamp())
    });
    let to = matches.value_of("to").map(|t| {
        Local
            .datetime_from_str(&format!("{} 00:00:00", t), "%d-%m-%Y %T")
            .map(|d| d.timestamp())
    });

    let mut query = String::from("SELECT * FROM entries");
    match (from, to) {
        (Some(f), Some(t)) => {
            query.push_str(&format!(" WHERE date > {} AND date < {}", f?, t?))
        }
        (Some(f), None) => query.push_str(&format!(" WHERE date > {}", f?)),
        (None, Some(t)) => query.push_str(&format!(" WHERE date < {}", t?)),
        (None, None) => (),
    }
    let statement = connection.prepare(query)?;
    let mut cursor = statement.cursor();

    while let Some(row) = cursor.next().unwrap() {
        let timestamp = row[2].as_integer().unwrap();
        let date =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        println!(
            "\x1b[1;35m# {}\x1b[0m",
            date.with_timezone(&Local)
            .format("%b %e %Y - %H:%M")
            .to_string()
            );
        println!(
            "{}",
            String::from_utf8(row[1].as_binary().unwrap().to_vec()).unwrap()
            );
    }

    Ok(())
}
