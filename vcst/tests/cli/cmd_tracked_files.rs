use crate::libtest::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::libtest::setup::{make_test_temp, vcs_test_setup, TestDirs, TestScope};
use assert_cmd::Command;
use predicates::prelude::*;

static TEST_SCOPE: TestScope = TestScope::new("cmd_tracked_files.rs");

#[test]
fn git() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).git_repo;

    //
    // Assert: nothing to track yet, so nutput
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_empty());

    //
    // Arrange: ensur ewe have  history to talk about the tacking of
    let tracked_files = vec!["git-license.txt", "git-readme.txt"];
    for basename in &tracked_files {
        let mut touched_file = test_dir.clone();
        touched_file.push(basename);
        make_test_temp::touch(touched_file.as_ref()).unwrap();
    }
    vcs_test_setup::run_cli_from_tempdir("git", vec!["add", "."].as_ref(), &test_dir).unwrap();
    vcs_test_setup::run_cli_from_tempdir(
        "git",
        vec![
            "commit",
            "--no-verify",
            "--message",
            "test arrange phase: ensuring git history",
        ]
        .as_ref(),
        &test_dir,
    )
    .unwrap();

    //
    // Assert: actual history to report, all committed files listed
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(tracked_files.join("\n") + "\n"));
}

#[test]
fn hg() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).hg_repo;

    //
    // Assert: nothing to track yet, so nutput
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_empty());

    //
    // Arrange: ensur ewe have  history to talk about the tacking of
    let tracked_files = vec!["hg-license.txt", "hg-readme.txt"];
    for basename in &tracked_files {
        let mut touched_file = test_dir.clone();
        touched_file.push(basename);
        make_test_temp::touch(touched_file.as_ref()).unwrap();
    }
    vcs_test_setup::run_cli_from_tempdir("hg", vec!["add", "."].as_ref(), &test_dir).unwrap();
    vcs_test_setup::run_cli_from_tempdir(
        "hg",
        vec![
            "commit",
            "--message",
            "test arrange phase: ensuring hg history",
        ]
        .as_ref(),
        &test_dir,
    )
    .unwrap();

    //
    // Assert: actual history to report, all committed files listed
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(tracked_files.join("\n") + "\n"));
}

#[test]
fn jj() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).jj_repo;

    //
    // Assert: nothing to track yet, so nutput
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_empty());

    //
    // Arrange: ensur ewe have  history to talk about the tacking of
    let tracked_files = vec!["jj-license.txt", "jj-readme.txt"];
    for basename in &tracked_files {
        let mut touched_file = test_dir.clone();
        touched_file.push(basename);
        make_test_temp::touch(touched_file.as_ref()).unwrap();
    }
    vcs_test_setup::run_cli_from_tempdir(
        "jj",
        vec![
            "commit",
            "--message",
            "test arrange phase: ensuring jj history",
        ]
        .as_ref(),
        &test_dir,
    )
    .unwrap();

    //
    // Assert: actual history to report, all committed files listed
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::diff(tracked_files.join("\n") + "\n"));
}

#[test]
fn novcs() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_vcs;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("tracked-files").arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS.to_string() + "\n"));
}

#[test]
fn non_dir() {
    let not_dir = &TestDirs::create_once(&TEST_SCOPE).not_dir;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("tracked-files").arg(not_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
}

#[test]
fn non_extant() {
    let test_dirs = &TestDirs::create_once(&TEST_SCOPE);
    let non_extant_path = test_dirs.non_extant();
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("tracked-files").arg(non_extant_path).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
}
