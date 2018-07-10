extern crate jot;

fn main() {
    let config = jot::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("There was a problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = jot::run(config) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

