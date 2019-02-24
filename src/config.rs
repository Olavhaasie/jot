use crate::cmd::Command;
use clap::{AppSettings, Arg, ArgGroup, ArgMatches};

pub struct Config<'a> {
    pub command: Command,
    pub matches: ArgMatches<'a>,
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
            Arg::with_name("json")
                .long("json")
                .help("outputs in json format"),
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
                .help(
                    "case insensitive pattern to look for inside journal entries. \
                     '_' can be used as wildcard character and '%' for one or more",
                ),
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .help("reverse the date output order"),
        ];

        let list_group = ArgGroup::with_name("list-mode")
            .args(&["list", "json", "count", "from", "to", "pattern", "reverse"])
            .required(false)
            .multiple(true);

        let matches = app_from_crate!()
            .setting(AppSettings::ColoredHelp)
            .max_term_width(100)
            .args(args)
            .group(list_group)
            .get_matches();

        let list = matches.is_present("list-mode");
        Config {
            command: if list { Command::List } else { Command::Insert },
            matches,
        }
    }
}
