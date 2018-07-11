use std::env;

pub const HELP_INFO: &'static str = "\
personal journal for command line

USAGE:
    jot [OPTIONS]

OPTIONS:
    -l, --list     List all journal entries and exit
    -f, --file     Journal file to add to/read from
    -V, --version  Print version
    -h, --help     Print this help information\
";

pub const DEFAULT_FILENAME: &'static str = "journal.txt";
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

pub enum Command {
    Help,
    Version,
    Edit,
    List,
}

pub struct Config {
    pub name: String,
    pub command: Command,
    pub filename: String,
    pub color: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let name = args.next().unwrap();

        let mut config = Config { name, command: Command::Edit, filename: DEFAULT_FILENAME.to_string(), color: true };
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "-h" | "--help" => {
                    config.command = Command::Help;
                    break;
                },
                "-V" | "--version" => {
                    config.command = Command::Version;
                    break;
                },
                "-l" | "--list" => config.command = Command::List,
                "-f" | "--file" => config.filename = args.next().unwrap(),
                _ => return Err("Unknown option. see --help"),
            }
        }
        Ok(config)
    }
}

