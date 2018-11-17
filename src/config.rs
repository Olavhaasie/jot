use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches,
};

const DEFAULT_FILENAME: &'static str = "journal.db";

pub enum Command {
    Insert,
    List,
}

pub struct Config<'a> {
    pub command: Command,
    pub matches: ArgMatches<'a>,
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        let matches = app_from_crate!()
            .arg(
                Arg::with_name("color")
                    .short("n")
                    .long("no-color")
                    .help("disables colored output"),
            ).arg(
                Arg::with_name("db")
                    .short("d")
                    .long("database")
                    .value_name("FILE")
                    .takes_value(true)
                    .default_value(DEFAULT_FILENAME)
                    .help("journal database to read from"),
            ).arg(
                Arg::with_name("list")
                    .short("l")
                    .long("list")
                    .help("lists all journal entries"),
            ).arg(
                Arg::with_name("from")
                    .short("f")
                    .long("from")
                    .value_name("DATE")
                    .takes_value(true)
                    .help("sets lower boundary date to retrieve journal entries"),
            ).arg(
                Arg::with_name("to")
                    .short("t")
                    .long("to")
                    .value_name("DATE")
                    .takes_value(true)
                    .help("sets upper boundary date to retrieve journal entries"),
            ).get_matches();

        let list =
            matches.is_present("list") || matches.is_present("from") || matches.is_present("to");
        Config {
            command: if list { Command::List } else { Command::Insert },
            matches,
        }
    }
}
