use assert_cmd::Command;

#[test]
fn test_init_command() {
    let mut cmd = Command::cargo_bin("cardano-devkit").unwrap();
    cmd.arg("init").assert().success();
}
