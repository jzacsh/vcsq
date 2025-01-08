use crate::libtest::consts::ERROR_DIR_MISSING;
use crate::libtest::setup::{TestDirs, TestScope};
use assert_cmd::Command;
use predicates::prelude::*;

static TEST_SCOPE: TestScope = TestScope::new("cli.rs");

/// TODO: (feature,cleap) fix CLI-clunkiness and make a global dir arg
#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin("vcsq").unwrap();
    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_DIR_MISSING.to_string() + "\n"));
}

#[test]
fn no_subcmd() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).git_repo;

    // Defaults to "brand" subcmd behavior
    let mut cmd = Command::cargo_bin("vcsq").unwrap();
    cmd.arg("--dir")
        .arg(test_dir)
        .assert()
        .success()
        .stdout(predicate::eq("Git\n"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn bare_dir() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).git_repo;
    // Prove our assert-phase reults won't be due to test-dir _not_ eixsitng (eg: due to some
    // test-hraness/setup failure).
    assert!(test_dir.exists());
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let assert = cmd.arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::starts_with(
            "error: unrecognized subcommand",
        ))
        .stderr(predicate::str::contains("Usage: vcsq [OPTIONS] [COMMAND]"))
        .stderr(predicate::str::contains("try '--help'"));
}
