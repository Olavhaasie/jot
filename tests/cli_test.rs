use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help_and_quit() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.args(&["--help"]);
    cmd.assert().success();
}
