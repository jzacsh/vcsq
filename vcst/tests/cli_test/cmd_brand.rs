use crate::common::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
use crate::common::setup::{TestDirs, TestScope};
use assert_cmd::Command;

use predicates::prelude::*;

mod cmd_brand {
    use super::*;
    static TEST_SCOPE: TestScope = TestScope::new("cmd_brand.rs");

    #[test]
    fn git() {
        let test_dir = &TestDirs::create_once(&TEST_SCOPE).git_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Git\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn hg() {
        let test_dir = &TestDirs::create_once(&TEST_SCOPE).hg_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Mercurial\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn jj() {
        let test_dir = &TestDirs::create_once(&TEST_SCOPE).jj_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Jujutsu\n"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn novcs() {
        let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_vcs;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS.to_string() + "\n"));
    }

    #[test]
    fn non_dir() {
        let test_dir = &TestDirs::create_once(&TEST_SCOPE).not_dir;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
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

        let assert = cmd.arg("brand").arg(non_extant_path).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR.to_string() + "\n"));
    }
}
