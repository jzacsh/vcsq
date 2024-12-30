mod common;
use crate::common::TestDirs;
use std::process::exit;
use std::sync::Once;

static ONE_REPO_SETUP: Once = Once::new();

static ERROR_NO_KNOWN_VCS: &'static str =
    "vcs error: if dir is a VCS, it\'s of an unknown brand (tried 2: [Git, Mercurial])";

static ERROR_NOT_VALID_DIR: &'static str = "usage error: dir must be a readable directory";

/// Setup tests, and ensure heavy operations aren't thrashing our disk (or ramdisk) more than once
/// a run.
///
/// See this log via nocapture flag:
/// ```rust
/// cargo test -- --nocapture
/// ```
fn setup_tests() -> TestDirs {
    use crate::common::make_test_temp::mktemp;

    let testdir_basename = "vcst-e2e-testdirs";
    ONE_REPO_SETUP.call_once(|| {
        let tmpdir_root = mktemp(testdir_basename).expect("setting up test dir");
        eprintln!("SETUP: {:?}", tmpdir_root.clone());
        match TestDirs::create(&tmpdir_root) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("test harness fail: {}", e);
                exit(1);
            }
        }
    });

    // TODO: (rust) how to capture the mktemp root out of this? we basically need
    // create() to return all three tempdirs it made (one PathBuf for each VCS repo
    // path).
    TestDirs::new(testdir_basename).expect("failed listing tempdirs")
}

mod brand {
    use crate::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn git() {
        let test_dir = crate::setup_tests().git_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Git"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn hg() {
        let test_dir = crate::setup_tests().hg_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .success()
            .stdout(predicate::eq("Mercurial"))
            .stderr(predicate::str::is_empty());
    }

    #[test]
    fn jj() {
        let test_dir = crate::setup_tests().jj_repo;
        assert_eq!(42, 42); // TODO: write real test
    }

    #[test]
    fn novcs() {
        let test_dir = crate::setup_tests().not_vcs;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(ERROR_NO_KNOWN_VCS.to_string()));
    }

    #[test]
    fn non_dir() {
        let test_dir = crate::setup_tests().not_dir;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(ERROR_NOT_VALID_DIR));
    }

    #[test]
    fn non_extant() {
        let test_dirs = crate::setup_tests();
        let non_extant_path = test_dirs.non_extant();
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let assert = cmd.arg("brand").arg(non_extant_path).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(ERROR_NOT_VALID_DIR));
    }
}

mod root {
    use crate::{ERROR_NOT_VALID_DIR, ERROR_NO_KNOWN_VCS};
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn git() {
        let test_dir = crate::setup_tests().git_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.to_str().expect("git repo path utf8");

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .success()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::eq(expected_root));
    }

    #[test]
    fn hg() {
        let test_dir = crate::setup_tests().hg_repo;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.to_str().expect("hg repo path utf8");

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .success()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::eq(expected_root));
    }

    #[test]
    fn jj() {
        let test_dir = crate::setup_tests().jj_repo;
        assert_eq!(42, 42); // TODO: write real test
    }

    #[test]
    fn novcs() {
        let test_dir = crate::setup_tests().not_vcs;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = test_dir.to_str().expect("non-vcs dir path utf8");

        let assert = cmd.arg("root").arg(&test_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(ERROR_NO_KNOWN_VCS.to_string()));
    }

    #[test]
    fn non_dir() {
        let not_dir = crate::setup_tests().not_dir;
        let mut cmd = Command::cargo_bin("vcst").unwrap();

        let expected_root = not_dir.to_str().expect("not-dir path utf8");

        let assert = cmd.arg("root").arg(&not_dir).assert();
        assert
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::diff(ERROR_NOT_VALID_DIR));
    }

    #[test]
    fn non_extant() {
        let mut non_extant = crate::setup_tests().non_extant();
        assert_eq!(42, 42); // TODO: write real test
    }
}

/// TODO: (feature,cleap) fix CLI-clunkiness and make a global dir arg
mod cli_edges {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn no_args() {
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        cmd.assert()
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(
                "usage error: require either subcmd with a query or a direct --dir",
            ));
    }

    #[test]
    fn no_subcmd() {
        assert_eq!(42, 42); // TODO: `--dir dir`
        assert_eq!(42, 42); // TODO: assert `--dir=DIR` is the same as `brand DIR`
    }

    #[test]
    fn bare_dir() {
        let test_dir = crate::setup_tests().git_repo;
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
