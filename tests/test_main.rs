use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*;

#[test]
fn test_run() {
    Command::new(cargo_bin!())
        .arg("--help")
        .unwrap()
        .assert()
        .success();
}
