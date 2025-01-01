use crate::common::setup::{make_test_temp, TestDirs, TestScope};
use assert_cmd::Command;
use libvcst::repo::ERROR_REPO_NOT_DIRTY;
use predicates::prelude::*;

mod cmd_dirty {
    use super::*;
    static TEST_SCOPE: TestScope = TestScope::new("cmd_dirty.rs");

    #[test]
    fn git() {
        let test_dirs = TestDirs::create_once(&TEST_SCOPE);
        let test_dir = &test_dirs.git_repo;

        //
        // Arrange+Assert: clean repo lists nothing dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd
            .arg("dirty-files")
            .arg("--clean-ok")
            .arg(&test_dir)
            .assert();
        assert
            .success()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty());

        //
        // Arrange+Assert: clean repo lists complains without --clean-ok
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::contains(ERROR_REPO_NOT_DIRTY));

        //
        // Arrange: make the repo dirty
        //
        let mut untracked_file = test_dir.clone();
        untracked_file.push("git-docs.md");

        make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");

        //
        // Assert: dirty repo now has report of what's dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .success()
            .stdout(predicate::str::diff("git-docs.md\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn hg() {
        let test_dirs = TestDirs::create_once(&TEST_SCOPE);
        let test_dir = &test_dirs.hg_repo;

        //
        // Arrange+Assert: clean repo lists nothing dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd
            .arg("dirty-files")
            .arg("--clean-ok")
            .arg(&test_dir)
            .assert();
        assert
            .success()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty());

        //
        // Arrange+Assert: clean repo lists complains without --clean-ok
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::contains(ERROR_REPO_NOT_DIRTY));

        //
        // Arrange: make the repo dirty
        //
        let mut untracked_file = test_dir.clone();
        untracked_file.push("mercurial.md");

        make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");

        //
        // Assert: dirty repo now has report of what's dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .success()
            .stdout(predicate::str::diff("mercurial.md\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn jj() {
        let test_dirs = TestDirs::create_once(&TEST_SCOPE);
        let test_dir = &test_dirs.jj_repo;

        //
        // Arrange+Assert: clean repo lists nothing dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd
            .arg("dirty-files")
            .arg("--clean-ok")
            .arg(&test_dir)
            .assert();
        assert
            .success()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty());

        //
        // Arrange+Assert: clean repo lists complains without --clean-ok
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::contains(ERROR_REPO_NOT_DIRTY));

        //
        // Arrange: make the repo dirty
        //
        let mut untracked_file = test_dir.clone();
        untracked_file.push("jj-vcs-docs.md");

        make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");

        //
        // Assert: dirty repo now has report of what's dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("dirty-files").arg(&test_dir).assert();
        assert
            .success()
            .stdout(predicate::str::diff("jj-vcs-docs.md\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn no_vcs() {
        assert_eq!(42, 42); // TODO: write test ERROR_NO_KNOWN_VCS
    }

    #[test]
    fn no_dir() {
        assert_eq!(42, 42); // TODO: write test ERROR_NOT_VALID_DIR
    }
}
