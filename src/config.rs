use crate::cmd::Command;
use clap::{Arg, ArgGroup, ArgMatches};

const DEFAULT_FILENAME: &str = "journal.sqlite";

pub struct Config<'a> {
    pub command: Command,
    pub matches: ArgMatches<'a>,
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Order {
        Asc,
        Desc,
    }
}

impl<'a> Config<'a> {
    pub fn default() -> Config<'a> {
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
            Arg::with_name("sort")
                .short("s")
                .long("sorted")
                .value_name("ORDER")
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&Order::variants())
                .default_value("ASC")
                .help("sort by date in ascending or descending order"),
        ];

        let list_group =
            ArgGroup::with_name("list-mode").args(&["list", "count", "from", "to", "pattern"]);

        let matches = app_from_crate!().args(args).group(list_group).get_matches();

        let list = matches.is_present("list-mode");
        Config {
            command: if list { Command::List } else { Command::Insert },
            matches,
        }
    }
}
