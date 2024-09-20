use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_init_command() {
    let mut cmd = Command::cargo_bin("cardano-devkit").unwrap();
    cmd.arg("init")
        .assert()
        .success()
        .stdout(contains("Init command not implemented yet"));
}
