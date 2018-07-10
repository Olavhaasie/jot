extern crate chrono;

use chrono::prelude::*;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Entry {
    pub date: DateTime<Local>,
    pub body: String,
}

pub struct Journal {
    pub entries: Vec<Entry>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.date.format("%b %e %Y - %H:%M").to_string(), self.body)
    }
}

impl fmt::Display for Journal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in self.entries.iter() {
            write!(f, "{}\n\n", e)?;
        }
        Ok(())
    }
}

impl Journal {
    pub fn new(filename: &str) -> Result<Journal, Box<Error>> {
        let mut entries: Vec<Entry> = Vec::new();
        let file = File::open(filename)?;

        let mut entry: Option<Entry> = None;
        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.starts_with("#") {
                let date: DateTime<Local> = line[1..].parse::<DateTime<Local>>()?;

                if entry.is_some() {
                    entries.push(entry.take().unwrap());
                }

                entry = Some(Entry { date, body: String::new() });

            } else {
                if let Some(l) = entry.as_mut() {
                    l.body += &line;
                }
            }
        }

        if entry.is_some() {
            entries.push(entry.take().unwrap());
        }
        Ok(Journal{ entries })
    }
}

