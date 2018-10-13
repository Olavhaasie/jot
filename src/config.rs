use std::env;

const DEFAULT_FILENAME: &'static str = "journal.db";
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

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

        let mut config = Config {
            name: NAME.unwrap_or(&name).to_string(),
            command: Command::Edit,
            filename: DEFAULT_FILENAME.to_string(),
            color: true,
        };
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "-h" | "--help" => {
                    config.command = Command::Help;
                    break;
                }
                "-V" | "--version" => {
                    config.command = Command::Version;
                    break;
                }
                "-l" | "--list" => config.command = Command::List,
                "-f" | "--file" => {
                    let a = args.next();
                    if a.as_ref().map_or(true, |f| f.starts_with("-")) {
                        return Err("'-f' requires a file name");
                    } else {
                        config.filename = a.unwrap();
                    }
                }
                _ => return Err("Unknown option. see --help"),
            }
        }
        Ok(config)
    }

    pub fn print_version(&self) {
        println!("{} {}", self.name, VERSION.unwrap_or("[unknown version]"));
    }

    pub fn print_help(&self) {
        println!(
            "\
personal journal for command line

USAGE:
{} [OPTIONS]

OPTIONS:
-l, --list          List all journal entries and exit
-f, --file <db>     Journal database to add to/read from
-V, --version       Print version
-h, --help          Print this help information

Jot has two modes: edit and list.
You can enter edit mode with no arguments, then you can start typing.
When finished writing you can press ^D (Control-D) to save the journal entry.
The list mode can be used by giving the '--list' option.",
            self.name
        );
    }
}
