use crate::libtest::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::libtest::setup::{TestDirs, TestScope};
use assert_cmd::Command;
use predicates::prelude::*;

static TEST_SCOPE: TestScope = TestScope::new("cmd_root.rs");

#[test]
fn git() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).git_repo;
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let expected_root = test_dir.display().to_string();

    let assert = cmd.arg("root").arg(&expected_root).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(expected_root + "\n"));
}

#[test]
fn hg() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).hg_repo;
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let expected_root = test_dir.display().to_string();

    let assert = cmd.arg("root").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(expected_root + "\n"));
}

#[test]
fn jj() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).jj_repo;
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let expected_root = test_dir.display().to_string();

    let assert = cmd.arg("root").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(expected_root + "\n"));
}
#[test]
fn novcs() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_vcs;
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let assert = cmd.arg("root").arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS.to_string() + "\n"));
}

#[test]
fn non_dir() {
    let not_dir = &TestDirs::create_once(&TEST_SCOPE).not_dir;
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let assert = cmd.arg("root").arg(not_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
}

#[test]
fn non_extant() {
    let test_dirs = &TestDirs::create_once(&TEST_SCOPE);
    let non_extant_path = test_dirs.non_extant();
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let assert = cmd.arg("root").arg(non_extant_path).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
}
