use crate::common::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::common::setup::TestDirs;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn git() {
    let test_dir = TestDirs::create_once().git_repo;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(test_dir).assert();
    assert
        .success()
        .stdout(predicate::eq("Git"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn hg() {
    let test_dir = TestDirs::create_once().hg_repo;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(test_dir).assert();
    assert
        .success()
        .stdout(predicate::eq("Mercurial"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn jj() {
    let test_dir = TestDirs::create_once().jj_repo;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(test_dir).assert();
    assert
        .success()
        .stdout(predicate::eq("Jujutsu"))
        .stderr(predicate::str::is_empty());
}
#[test]
fn novcs() {
    let test_dir = TestDirs::create_once().not_vcs;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS));
}

#[test]
fn non_dir() {
    let test_dir = TestDirs::create_once().not_dir;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
}

#[test]
fn non_extant() {
    let test_dirs = TestDirs::create_once();
    let non_extant_path = test_dirs.non_extant();
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("brand").arg(non_extant_path).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
}
