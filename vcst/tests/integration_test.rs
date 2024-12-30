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
    #[test]
    fn git() {
        let test_dir = crate::setup_tests().git_repo;
        // TODO: (rust) see if
        // https://rust-cli.github.io/book/tutorial/testing.html#testing-cli-applications-by-running-them
        // has tips on how to get access to run_fs_query(); Perhaps just move it to src/lib.rs or
        // something silly? or make a proper cli/ mod directory that tests can then access?
        //vcst::vcst_query(42, 42);
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
