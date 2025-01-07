use crate::libtest::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::libtest::setup::{make_test_temp, vcs_test_setup, TestDirs, TestScope};
use assert_cmd::Command;
use predicates::prelude::*;

static TEST_SCOPE: TestScope = TestScope::new("cmd_current_id.rs");

#[test]
fn git() {
    let test_dir = TestDirs::create_once(&TEST_SCOPE).git_repo;

    //
    // Assert: repo now no real current id, because it has not history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(&test_dir).assert();
    assert
        .success()
        .stdout(predicate::str::diff(
            "00000000000000000000000000000000000000000000000000\n",
        ))
        .stderr(predicate::str::is_empty());

    //
    // Arrange: add a commit to the repo's history
    //
    let mut untracked_file = test_dir.clone();
    untracked_file.push("git-unclean.md");
    make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");
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
    // Assert: repo now has a current id, because it has history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(&test_dir).assert();
    assert
        .success()
        // TODO: need a deterministic test here; maybe tarball a few repos (with interestig
        // history) as our "test data", and unpack those to temp so we can test against their
        // preconfigured state?
        .stdout(predicate::str::is_empty().not())
        .stderr(predicate::str::is_empty());
}

#[test]
fn hg() {
    let test_dir = TestDirs::create_once(&TEST_SCOPE).hg_repo;

    //
    // Assert: repo now no real current id, because it has not history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(&test_dir).assert();
    assert
        .success()
        .stdout(predicate::str::diff(
            "0000000000000000000000000000000000000000\n",
        ))
        .stderr(predicate::str::is_empty());

    //
    // Arrange: add a commit to the repo's history
    //
    let mut untracked_file = test_dir.clone();
    untracked_file.push("mercurial-unclean.md");
    make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");
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
    // Assert: repo now has a current id, because it has history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(&test_dir).assert();
    assert
        .success()
        // TODO: need a deterministic test here; maybe tarball a few repos (with interestig
        // history) as our "test data", and unpack those to temp so we can test against their
        // preconfigured state?
        .stdout(predicate::str::is_empty().not())
        .stderr(predicate::str::is_empty());
}

#[test]
fn jj() {
    let test_dir = TestDirs::create_once(&TEST_SCOPE).jj_repo;

    //
    // Assert: repo now no real current id, because it has not history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(&test_dir).assert();
    assert
        .success()
        .stdout(predicate::str::diff(
            "0000000000000000000000000000000000000000\n",
        ))
        .stderr(predicate::str::is_empty());

    //
    // Arrange: add a commit to the repo's history
    //
    let mut untracked_file = test_dir.clone();
    untracked_file.push("jj-vcs-unclean.md");
    make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");
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
    // Assert: repo now has a current id, because it has history
    //
    let mut cmd = Command::cargo_bin("vcst").unwrap();
    let assert = cmd.arg("current-id").arg(test_dir).assert();
    assert
        .success()
        // TODO: need a deterministic test here; maybe tarball a few repos (with interestig
        // history) as our "test data", and unpack those to temp so we can test against their
        // preconfigured state?
        .stdout(predicate::str::is_empty().not())
        .stderr(predicate::str::is_empty());
}

#[test]
fn novcs() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_vcs;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("current-id").arg(test_dir).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS.to_string() + "\n"));
}

#[test]
fn non_dir() {
    let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_dir;
    let mut cmd = Command::cargo_bin("vcst").unwrap();

    let assert = cmd.arg("current-id").arg(test_dir).assert();
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

    let assert = cmd.arg("current-id").arg(non_extant_path).assert();
    assert
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
}
