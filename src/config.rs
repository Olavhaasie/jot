use super::cmd::Command;

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches,
};

const DEFAULT_FILENAME: &'static str = "journal.db";

pub struct Config<'a> {
    pub command: Command,
    pub matches: ArgMatches<'a>,
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        let args = &[
            Arg::with_name("nocolor")
                .long("no-color")
                .help("disables colored output"),
            Arg::with_name("db")
                .short("d")
                .long("database")
                .value_name("FILE")
                .takes_value(true)
                .default_value(DEFAULT_FILENAME)
                .help("journal database to read from"),
            Arg::with_name("editor")
                .short("e")
                .long("editor")
                .value_name("CMD")
                .takes_value(true)
                .help("editor to use when inserting a new journal entry"),
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("lists all journal entries"),
            Arg::with_name("count")
                .short("n")
                .long("entries")
                .value_name("NUM")
                .takes_value(true)
                .help("limits the amount of latest entries"),
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("DATE")
                .takes_value(true)
                .help("sets lower boundary date to retrieve journal entries"),
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("DATE")
                .takes_value(true)
                .help("sets upper boundary date to retrieve journal entries"),
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .value_name("PATTERN")
                .takes_value(true)
                .help("case insensitive pattern to look for inside journal entries. '_' can be used as wildcard character and '%' for one ore more"),
        ];

        let matches = app_from_crate!().args(args).get_matches();

        let list = matches.is_present("list")
            || matches.is_present("count")
            || matches.is_present("from")
            || matches.is_present("to")
            || matches.is_present("pattern");
        Config {
            command: if list { Command::List } else { Command::Insert },
            matches,
        }
    }
}
