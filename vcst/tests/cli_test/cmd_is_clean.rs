use crate::common::setup::{make_test_temp, TestDirs, TestScope};
use assert_cmd::Command;
use libvcst::repo::ERROR_REPO_NOT_DIRTY;
use predicates::prelude::*;

mod cmd_is_clean {
    use super::*;
    static TEST_SCOPE: TestScope = TestScope::new("cmd_is_clean.rs");

    #[test]
    fn git() {
        assert_eq!(42, 42); // TODO: write test
    }

    #[test]
    fn hg() {
        let test_dirs = TestDirs::create_once(&TEST_SCOPE);
        let test_dir = &test_dirs.hg_repo;

        //
        // Arrange+Assert: clean repo lists nothing dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("is-clean").arg(&test_dir).assert();
        assert
            .success()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty());

        //
        // Arrange: make the repo dirty
        //
        let mut untracked_file = test_dir.clone();
        untracked_file.push("quick-start.md");

        make_test_temp::touch(&untracked_file).expect("test arrange: touch failed");

        //
        // Assert: dirty repo now has report of what's dirty
        //
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        let assert = cmd.arg("is-clean").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn jj() {
        assert_eq!(42, 42); // TODO: write test
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
