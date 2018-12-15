use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help_and_quit() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.args(&["--help"]);
    cmd.assert().success();
}

#[test]
fn create_and_insert() {
    let mut cmd = Command::main_binary().unwrap();
    let file = tempfile::Builder::new()
        .tempfile_in("./")
        .expect("failed to create tempfile in current directory");
    cmd.args(&["-d", file.path().to_str().unwrap()])
        .with_stdin()
        .buffer("test entry");
    cmd.assert().success();
}
