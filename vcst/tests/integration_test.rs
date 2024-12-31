mod common;
use common::setup::TestDirs;

mod brand {
    use crate::common::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn git() {
        let test_dir = crate::TestDirs::create_once().git_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Git"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn hg() {
        let test_dir = crate::TestDirs::create_once().hg_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Mercurial"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn jj() {
        let test_dir = crate::TestDirs::create_once().jj_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Jujutsu"))
            .stderr(predicate::str::is_empty());
    }
    #[test]
    fn novcs() {
        let test_dir = crate::TestDirs::create_once().not_vcs;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS));
    }

    #[test]
    fn non_dir() {
        let test_dir = crate::TestDirs::create_once().not_dir;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
    }

    #[test]
    fn non_extant() {
        let test_dirs = crate::TestDirs::create_once();
        let non_extant_path = test_dirs.non_extant();
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(non_extant_path).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
    }
}

mod root {
    use crate::common::consts::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn git() {
        let test_dir = crate::TestDirs::create_once().git_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.display().to_string();

        let assert = cmd.arg("root").arg(&expected_root).assert();
        assert
            .success()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::diff(expected_root));
    }

    #[test]
    fn hg() {
        let test_dir = crate::TestDirs::create_once().hg_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.display().to_string();

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .success()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::diff(expected_root));
    }

    #[test]
    fn jj() {
        let test_dir = crate::TestDirs::create_once().jj_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.display().to_string();

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .success()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::diff(expected_root));
    }
    #[test]
    fn novcs() {
        let test_dir = crate::TestDirs::create_once().not_vcs;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NO_KNOWN_VCS));
    }

    #[test]
    fn non_dir() {
        let not_dir = crate::TestDirs::create_once().not_dir;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("root").arg(&not_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
    }

    #[test]
    fn non_extant() {
        let test_dirs = crate::TestDirs::create_once();
        let non_extant_path = test_dirs.non_extant();
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("root").arg(non_extant_path).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
    }
}

/// TODO: (feature,cleap) fix CLI-clunkiness and make a global dir arg
mod cli_edges {
    use crate::common::consts::ERROR_DIR_MISSING;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn no_args() {
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        cmd.assert()
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_DIR_MISSING));
    }

    #[test]
    fn no_subcmd() {
        let test_dir = crate::TestDirs::create_once().git_repo;

        // Defaults to "brand" subcmd behavior
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        cmd.arg("--dir")
            .arg(test_dir)
            .assert()
            .success()
            .stdout(predicate::eq("Git"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn bare_dir() {
        let test_dir = crate::TestDirs::create_once().git_repo;
        // Prove our assert-phase reults won't be due to test-dir _not_ eixsitng (eg: due to some
        // test-hraness/setup failure).
        assert!(test_dir.exists());
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::starts_with(
                "error: unrecognized subcommand",
            ))
            .stderr(predicate::str::contains("Usage: vcst [OPTIONS] [COMMAND]"))
            .stderr(predicate::str::contains("try '--help'"));
    }
}
