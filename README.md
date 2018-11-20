<p align="center">
```
 _____           __
/\___ \         /\ \__
\/__/\ \    ___ \ \ ,_\
   _\ \ \  / __`\\ \ \/
  /\ \_\ \/\ \L\ \\ \ \_
  \ \____/\ \____/ \ \__\
   \/___/  \/___/   \/__/
```
A terminal based digital personal journal 📔
</p>

# Help

## Build
Jot is written in Rust and therefore uses
[Cargo](https://doc.rust-lang.org/cargo) as build tool.
To build jot run

    $ cargo build

To test run it

    $ cargo run

To build for release

    $ cargo build --release

The executable will be placed in `target/release/jot`.
There is also an option to install jot

    $ cargo install --path .

This will place jot inside `~/.cargo/bin/`.

## Usage
Jot has two modes: writing and listing.
To enter writing mode simply do

    $ jot

Jot will now accept input.
To cancel press control-c and to save the entry press control-d (EOF).
The following commands all insert a new entry to the default journal.

    $ echo "My first entry" | jot
    $ jot <<< "I bought a new chair"
    $ jot < entry.txt
    $ jot --editor vim

The last command opens an editor (vim in this case), where you can type
your journal entry.
After you saved and successfully exited the editor will the entry be saved
to your journal.
Jot also supports Unicode characters (because Rust does 😊).

    $ jot <<< "내 첫 번째 항목"

Besides from creating new entries, jot can also list your current entries.
You can list all your entries.

    $ jot -l

However, this is not very useful when you are looking for a specific entry.
That's why you can filter on content, dates and number of entries.

    $ jot --pattern study
    _lists all entries where study occurs... (case insensitive)_
    $ jot --from 20-11-2018
    _lists all entries from (inclusive) 20 November 2018..._
    $ jot -n 10 --to 25-10-2018
    _lists the 10 latest entries until 25 October 2018..._

The `-l` or `--list` argument is implied when using any of the filters.

# License
This software is distributed under MIT license 📝
