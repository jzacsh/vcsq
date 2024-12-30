mod common;
use crate::common::TestDirs;
use std::process::exit;
use std::sync::Once;

static ONE_REPO_SETUP: Once = Once::new();

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
    use vcst::{vcst_query, VcstArgs};

    #[test]
    fn git() {
        //
        // Arrange
        //
        let test_dir = crate::setup_tests().git_repo;
        //let mut fake_stdout = String::new();
        //let mut fake_stderr = String::new();
        //let vcst_args: VcstArgs = todo!();

        assert_eq!(42, 42);
        // vcst_query(V, fake_stdout, fake_stderr);
    }

    #[test]
    fn hg() {
        let test_dir = crate::setup_tests().hg_repo;
        assert_eq!(42, 42);
    }

    #[test]
    fn jj() {
        let test_dir = crate::setup_tests().jj_repo;
        assert_eq!(42, 42);
    }

    #[test]
    fn novcs() {
        let _ = crate::setup_tests().not_vcs; /* DO NOT SUBMIT - add plain_dir */
        assert_eq!(42, 42);
    }

    #[test]
    fn non_dir() {
        let _ = crate::setup_tests().not_dir; /* DO NOT SUBMIT - add plain_dir */
        assert_eq!(42, 42);
    }

    #[test]
    fn non_extant() {
        let mut non_extant = crate::setup_tests().non_extant();
        assert_eq!(42, 42);
    }
}

mod root {
    use vcst::{vcst_query, VcstArgs};

    #[test]
    fn git() {
        let test_dir = crate::setup_tests().git_repo;
        assert_eq!(42, 42);
    }

    #[test]
    fn hg() {
        let test_dir = crate::setup_tests().hg_repo;
        assert_eq!(42, 42);
    }

    #[test]
    fn jj() {
        let test_dir = crate::setup_tests().jj_repo;
        assert_eq!(42, 42);
    }

    #[test]
    fn novcs() {
        let _ = crate::setup_tests().not_vcs; /* DO NOT SUBMIT - add plain_dir */
        assert_eq!(42, 42);
    }

    #[test]
    fn non_dir() {
        let _ = crate::setup_tests().not_dir; /* DO NOT SUBMIT - add plain_dir */
        assert_eq!(42, 42);
    }

    #[test]
    fn non_extant() {
        let mut non_extant = crate::setup_tests().non_extant();
        assert_eq!(42, 42);
    }
}

/// TODO: (feature,cleap) fix CLI-clunkiness and make a global dir arg
mod cli_edges {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn no_args() {
        let mut cmd = Command::cargo_bin("vcst").unwrap();
        //
        // Assert
        //
        cmd.assert()
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::eq(
                "usage error: require either subcmd with a query or a direct --dir",
            ));
    }

    #[test]
    fn no_subcmd() {
        assert_eq!(42, 42); // `--dir dir`
        assert_eq!(42, 42); // assert `--dir=DIR` is the same as `brand DIR`
    }

    #[test]
    fn bare_dir() {
        assert_eq!(42, 42); // --dir required
    }
}
