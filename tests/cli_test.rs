use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help_and_quit() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&["--help"]);
    cmd.assert().success();
}
