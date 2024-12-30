mod common;
use std::sync::Once;

static ONE_REPO_SETUP: Once = Once::new();

/// Setup tests, and ensure heavy operations aren't thrashing our disk (or ramdisk) more than once
/// a run.
///
/// See this log via nocapture flag:
/// ```rust
/// cargo test -- --nocapture
/// ```
fn setup_tests() {
    ONE_REPO_SETUP.call_once(|| {
        let tmpdir_root = crate::common::mktemp("vcst-e2e-testdirs").expect("setting up test dir");
        eprintln!("test setup: {:?}", tmpdir_root.clone());
        crate::common::setup_temp_repos(&tmpdir_root).expect("setup temp repos");
    });
}

mod brand {
    #[test]
    fn git() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }

    #[test]
    fn hg() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }

    #[test]
    fn novcs() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }
}

mod root {
    #[test]
    fn git() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }

    #[test]
    fn hg() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }

    #[test]
    fn novcs() {
        crate::setup_tests();
        assert_eq!(42, 42);
    }
}
