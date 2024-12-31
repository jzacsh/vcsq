use crate::common::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::common::setup::{make_test_temp, TestDirs};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn git() {
    assert_eq!(42, 42); // TODO: write test
}

#[test]
fn hg_clean() {
    let test_dirs = TestDirs::create_once();
    let test_dir = test_dirs.hg_repo;

    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
    assert
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    //
    // Arrange: make the repo dirty
    //
    let mut untracked_file = test_dir.clone();
    untracked_file.push("MY_DOC.md");

    let _ = make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");

    //
    // Assert: dirty repo how has report of what's dirty
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
    assert
        .success()
        .stdout(predicate::str::diff("? MY_DOC.md\n"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn jj() {
    assert_eq!(42, 42); // TODO: write test
}
