use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    let mut cmd = Command::cargo_bin("umbrella").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn binary_with_location_but_no_api_key_gives_error() {
    let mut cmd = Command::cargo_bin("umbrella").unwrap();
    cmd.args(["London, UK"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"))
        .stderr(predicate::str::contains("api-key"))
        .stderr(predicate::str::contains("required"));
}

#[test]
#[ignore = "needs an API key and network access"]
fn binary_with_location_and_api_key_makes_real_api_request() {
    let mut cmd = Command::cargo_bin("umbrella").unwrap();
    cmd.args(["London, UK"])
        .assert()
        .success()
        .stdout(predicate::str::contains("London, United Kingdom"));
}
