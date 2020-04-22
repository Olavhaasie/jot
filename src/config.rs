use crate::cmd::Command;
use chrono::{prelude::*, ParseResult};
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_date(s: &str) -> ParseResult<i64> {
    Local
        .datetime_from_str(&format!("{} 00:00:00", s), "%d-%m-%Y %T")
        .map(|d| d.timestamp())
}

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Config {
    #[structopt(long = "no-color")]
    /// Disables colored output.
    pub no_color: bool,

    #[structopt(short = "d", long = "database", parse(from_os_str))]
    /// Journal database to read entries from.
    pub database: Option<PathBuf>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    /// Editor to use when inserting a new journal entry.
    pub editor: Option<String>,

    #[structopt(short = "l", long = "list")]
    /// Lists all journal entries.
    pub list: bool,

    #[structopt(long = "json")]
    /// Outputs in JSON format.
    pub json: bool,

    #[structopt(short = "n")]
    /// Limits the amount of latest entries.
    pub count: Option<u64>,

    #[structopt(short = "f", long = "from", parse(try_from_str = parse_date))]
    /// Sets lower boundary date to retrieve journal entries.
    pub from: Option<i64>,

    #[structopt(short = "t", long = "to", parse(try_from_str = parse_date))]
    /// Sets upper boundary date to retrieve journal entries.
    pub to: Option<i64>,

    #[structopt(short = "p", long = "pattern")]
    /// Case insensitive pattern to look for inside journal entries.
    /// '_' can be used as wildcard character and '%' for one or more
    pub pattern: Option<String>,

    #[structopt(short = "r", long = "reverse")]
    /// Reverse the date output order.
    pub reverse: bool,
}

impl Config {
    pub fn command(&self) -> Command {
        let list = self.list
            || self.json
            || self.count.is_some()
            || self.from.is_some()
            || self.to.is_some()
            || self.pattern.is_some()
            || self.reverse;

        if list {
            Command::List
        } else {
            Command::Insert
        }
    }
}
