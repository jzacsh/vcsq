use std::path::{Path, PathBuf};
use std::sync::Once;
use thiserror::Error;

pub static TEST_VCS_BASENAME_GIT: &str = "test-git-repo";
pub static TEST_VCS_BASENAME_HG: &str = "test-hg-repo";
pub static TEST_VCS_BASENAME_JJ: &str = "test-jj-on-git-repo";
pub static TEST_VCS_BASENAME_NONVCS: &str = "test-not-vcs";
pub static TEST_VCS_BASENAME_NONDIR: &str = "test-not-dir";
pub static TEST_SUBDIR_NAME_SUFFIX: &str = "testscope";

/// environment variable to optionally override the root (standard `$TMPDIR` or /tmp/) under which
/// the tests' temp directory tree is created/destroyed.
static ENVVAR_OVERRIDE_TESTDIR_ROOT: &str = "VCST_TESTDIR";

/// Directory under which all files are managed. This directory may itself be sub-contained (say in
/// /tmp/foo, depending on `ENVVAR_OVERRIDE_TESTDIR_ROOT` being present).
static TESTDIR_TMPDIR_ROOT: &str = "vcst-e2e-testdirs";

// TODO: (rust) how much of this file do the following two crates make obsolete/deletable?
// - https://docs.rs/assert_cmd/latest/assert_cmd
// - https://docs.rs/predicates/latest/predicates

#[derive(Error, Debug)]
pub enum TestSetupError {
    #[error("test harness: {}: {}", .context, .source)]
    RequiredCmd {
        context: String,
        source: Box<dyn std::error::Error>,
    },

    #[error("test harness: system err: {}: {}", .context, .source)]
    System {
        context: String,
        source: std::io::Error,
    },

    #[error("test harness: {0}")]
    Generic(String),
}

impl From<String> for TestSetupError {
    fn from(item: String) -> Self {
        TestSetupError::Generic(item)
    }
}

#[derive(Debug)]
pub struct TestDirs {
    pub root_dir: PathBuf,
    pub git_repo: PathBuf,
    pub hg_repo: PathBuf,
    pub jj_repo: PathBuf,
    pub not_vcs: PathBuf,
    pub not_dir: PathBuf,
}

pub struct TestScope {
    pub setup_idempotence: Once,
    pub test_name: &'static str,
}

impl TestScope {
    pub const fn new(test_name: &'static str) -> Self {
        Self {
            test_name,
            setup_idempotence: Once::new(),
        }
    }
    pub fn find_rootdir(&self, testdir_bname: &str) -> Result<PathBuf, TestSetupError> {
        TestDirs::list_temp_repos(testdir_bname, self)
    }
}

// TODO: (rust) figure out why this isn't actually dropping (neither stderr debug lines nor the
// actual directory cleanup appear to be happening.
impl Drop for TestScope {
    fn drop(&mut self) {
        let testrun_rootdir = self
            .find_rootdir(TESTDIR_TMPDIR_ROOT)
            .expect("test cleanup: failed dropping latest testdirs");
        eprintln!("dropping TestDirs root: {testrun_rootdir:?}");
        let _ = std::fs::remove_dir_all(testrun_rootdir);
    }
}

impl TestDirs {
    /// Reads from disk to find the latest temp directory tree.
    ///
    /// WARNING: will fail if `create_once()` hasn't been called at least once.
    fn new(testdir_bname: &str, scope: &TestScope) -> Result<Self, TestSetupError> {
        use std::path::Path;

        let root_dir = scope.find_rootdir(testdir_bname)?;
        let mut git_repo = root_dir.clone();
        git_repo.push(TEST_VCS_BASENAME_GIT);
        assert!(Path::exists(&git_repo), "git_repo missing: {:?}", &git_repo);

        let mut hg_repo = root_dir.clone();
        hg_repo.push(TEST_VCS_BASENAME_HG);
        assert!(Path::exists(&hg_repo), "hg_repo missing: {:?}", &hg_repo);

        let mut jj_repo = root_dir.clone();
        jj_repo.push(TEST_VCS_BASENAME_JJ);
        assert!(Path::exists(&jj_repo), "jj_repo missing: {:?}", &jj_repo);

        let mut not_vcs = root_dir.clone();
        not_vcs.push(TEST_VCS_BASENAME_NONVCS);
        assert!(Path::exists(&not_vcs), "not_vcs missing: {:?}", &not_vcs);

        let mut not_dir = root_dir.clone();
        not_dir.push(TEST_VCS_BASENAME_NONDIR);
        assert!(Path::exists(&not_dir), "not_dir missing: {:?}", &not_dir);

        Ok(TestDirs {
            root_dir,
            git_repo,
            hg_repo,
            jj_repo,
            not_vcs,
            not_dir,
        })
    }

    /// Mutates disk to start temp directory tree.
    fn create(testdir_bname: &Path) -> Result<(), TestSetupError> {
        use vcs_test_setup::setup_temp_repos;
        setup_temp_repos(testdir_bname)
    }

    /// Finds the latest tempdir on disk given `setup_temp_repos()` was called with `testdir_bname`
    fn list_temp_repos(testdir_bname: &str, scope: &TestScope) -> Result<PathBuf, TestSetupError> {
        use std::fs;

        let generic_root = make_test_temp::get_mktemp_root(testdir_bname)?;
        let mut test_run_dirs = fs::read_dir(&generic_root)
            // TODO: (rust) consider extracting[1] out theses errors (they get dropped via
            // filter_map() later) so we can err _iff_ there are zero OK results.
            // [1]: https://doc.rust-lang.org/rust-by-example/error/iter_result.html#collect-the-failed-items-with-map_err-and-filter_map
            .map_err(|source| TestSetupError::System {
                context: format!("fs::read_dir({})", generic_root.to_string_lossy()),
                source,
            })?
            .map(|res| res.map(|p| p.path()))
            .filter_map(std::result::Result::ok)
            .filter(|p| p.is_dir())
            .filter(|p| {
                p.to_string_lossy()
                    .trim_end()
                    .ends_with(&make_test_temp::testname_subdir_suffix(scope.test_name))
            })
            .collect::<Vec<_>>();
        test_run_dirs.sort();
        let newest_test_dir = test_run_dirs.into_iter().last().ok_or(format!(
            "list_temp_repos({}) no dirs found under: {}",
            testdir_bname,
            generic_root.to_string_lossy()
        ))?;
        Ok(newest_test_dir)
    }

    /// Generate a filepath we're pretty sure weon't exist (but in case some SUT tries to touch
    /// it, contain its path within `TestDirs`).
    pub fn non_extant(&self) -> PathBuf {
        use uuid::Uuid;
        let mut path = self.root_dir.clone();
        assert!(
            self.root_dir.clone().exists(),
            "test-harness bug: root dir should exist"
        );
        let uuidv4 = Uuid::new_v4();
        path.push(uuidv4.simple().to_string());
        assert!(
            !path.exists(),
            "test-harness bug: tried to gen rando flie, but got real one: {path:?}"
        );
        path
    }

    /// Idempotently setup tests, and ensure heavy operations aren't thrashing our disk (or
    /// ramdisk) more than once a run.
    ///
    /// See this log via nocapture flag:
    /// ```rust
    /// cargo test -- --nocapture
    /// ```
    ///
    /// `Self.new()` can be called endlessly to check the filesystem for the last run's result.
    pub fn create_once(test_scope: &TestScope) -> TestDirs {
        use make_test_temp::mktemp;
        use std::process::exit;

        test_scope.setup_idempotence.call_once(|| {
            let tmpdir_root =
                mktemp(TESTDIR_TMPDIR_ROOT, test_scope).expect("setting up test dir");
            eprintln!("SETUP: {:?}", tmpdir_root.clone());
            match TestDirs::create(&tmpdir_root) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("test harness fail: {e}");
                    exit(1);
                }
            }
        });

        // TODO: (rust) how to capture the mktemp root out of this? we basically need
        // create() to return all three tempdirs it made (one PathBuf for each VCS repo
        // path).
        Self::new(TESTDIR_TMPDIR_ROOT, test_scope).expect("failed listing tempdirs")
    }
}

pub mod vcs_test_setup {
    use super::TestSetupError;
    use super::{
        TEST_VCS_BASENAME_GIT, TEST_VCS_BASENAME_HG, TEST_VCS_BASENAME_JJ,
        TEST_VCS_BASENAME_NONDIR, TEST_VCS_BASENAME_NONVCS,
    };
    use std::path::Path;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    fn run_cli_from_tempdir(
        cmd: &str,
        args: Vec<&str>,
        tmpdir_root: &PathBuf,
    ) -> Result<(), TestSetupError> {
        let context_map = || {
            format!(
                "`{} {:?}` at {}",
                &cmd,
                &args,
                &tmpdir_root.to_string_lossy()
            )
        };
        let cli_output = Command::new(cmd)
            .args(&args)
            .stdout(Stdio::null())
            .current_dir(&tmpdir_root)
            .output()
            .map_err(|source| TestSetupError::System {
                context: context_map(),
                source,
            })?;
        if cli_output.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8(cli_output.stderr)
            .map_err(|source| TestSetupError::RequiredCmd {
                context: context_map(),
                source: Box::from(source),
            })?
            .trim()
            .to_string();
        Err(TestSetupError::RequiredCmd {
            context: context_map(),
            source: Box::from(format!("stderr: {stderr}")),
        })
    }

    fn setup_temp_repo_git(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("git", vec!["init", TEST_VCS_BASENAME_GIT], tmpdir_root)
    }

    fn setup_temp_repo_hg(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("hg", vec!["init", TEST_VCS_BASENAME_HG], tmpdir_root)
    }

    fn setup_temp_repo_jj(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("jj", vec!["git", "init", TEST_VCS_BASENAME_JJ], tmpdir_root)
    }

    fn setup_temp_nonvcs_dir(mut tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        use std::fs::create_dir;
        tmpdir_root.push(TEST_VCS_BASENAME_NONVCS);
        create_dir(&tmpdir_root).map_err(|source| TestSetupError::System {
            context: format!(
                "temp_nonvcs_dir({}): create_dir",
                tmpdir_root.to_string_lossy()
            ),
            source,
        })?;
        Ok(())
    }

    fn setup_temp_plainfile(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
        use super::make_test_temp::touch;

        let mut plain_file = tmpdir_root.clone();
        plain_file.push(TEST_VCS_BASENAME_NONDIR);
        touch(plain_file.as_path())
    }
    /// Creates new temp directories on disk.
    pub fn setup_temp_repos(tmpdir_root: &Path) -> Result<(), TestSetupError> {
        setup_temp_repo_git(&tmpdir_root.to_path_buf())?;
        setup_temp_repo_hg(&tmpdir_root.to_path_buf())?;
        setup_temp_repo_jj(&tmpdir_root.to_path_buf())?;
        setup_temp_nonvcs_dir(tmpdir_root.to_path_buf())?;
        setup_temp_plainfile(&tmpdir_root.to_path_buf())?;
        Ok(())
    }
}

/// Produce a directory path as defined by the set environment variable value by the name
/// `env_var`, or fallack to a safe system temporary directory.
/// TODO: cordon this off to a local "not stdlib", somewhere else.
fn env_dir_or_tmp(env_var: &str) -> Result<PathBuf, String> {
    use env::VarError;
    use std::env;

    match env::var(env_var) {
        Ok(d) => Ok(PathBuf::from(d)),
        Err(e) => match e {
            VarError::NotUnicode(source) => {
                Err(source.to_string_lossy().to_string())
            }
            VarError::NotPresent => Ok(env::temp_dir()),
        },
    }
}

pub mod make_test_temp {
    use super::{
        env_dir_or_tmp, TestScope, TestSetupError, ENVVAR_OVERRIDE_TESTDIR_ROOT,
        TEST_SUBDIR_NAME_SUFFIX,
    };
    use std::path::{Path, PathBuf};

    pub fn get_mktemp_root(basename: &str) -> Result<PathBuf, TestSetupError> {
        use std::fs::create_dir;

        let mut root_dir = env_dir_or_tmp(ENVVAR_OVERRIDE_TESTDIR_ROOT).map_err(|source| {
            TestSetupError::RequiredCmd {
                context: format!("env var ${ENVVAR_OVERRIDE_TESTDIR_ROOT}: bad utf8"),
                source: Box::from(source),
            }
        })?;

        // TODO: (rust) this feels ugly; is there a better way? what we're doing in the
        // ok_or_else() lines below: these are like assert!()'s, but don't panic, and allow
        // Rust to From<String> into my own error type

        root_dir
            .exists()
            .then_some(())
            .ok_or_else(|| format!("expect temp root_dir exists: {root_dir:?}"))?;

        root_dir
            .is_dir()
            .then_some(())
            .ok_or_else(|| format!("expect temp root_dir is dir: {root_dir:?}"))?;

        (root_dir != PathBuf::from("/"))
            .then_some(())
            .ok_or_else(|| format!("expect temp root_dir is not root: {root_dir:?}"))?;

        root_dir.push(basename);
        if !root_dir.exists() {
            create_dir(&root_dir).map_err(|source| TestSetupError::System {
                context: format!("get_mktemp_root({basename})"),
                source,
            })?;
        }
        Ok(root_dir)
    }

    /// How the hell is there no stdlib-esque functino for this??
    pub fn mktemp(basename: &str, test_scope: &TestScope) -> Result<PathBuf, TestSetupError> {
        use std::fs::create_dir;
        use uuid::Uuid;

        let mut root_dir: PathBuf = get_mktemp_root(basename)?;

        let iso_str = now_stamp("%F-at-%s");
        let test_name = test_scope.test_name.to_string();
        let dir_str = format!(
            "{}_{}_{}",
            iso_str,
            Uuid::new_v4().simple(),
            testname_subdir_suffix(&test_name),
        );
        root_dir.push(dir_str);
        create_dir(&root_dir).map_err(|source| TestSetupError::System {
            context: format!(
                "mktemp({}): create_dir({})",
                basename,
                root_dir.to_string_lossy()
            ),
            source,
        })?;

        assert!(
            root_dir.exists(),
            "bug: root_dir doesn't exist after creation: {root_dir:?}"
        );
        Ok(root_dir)
    }

    pub fn testname_subdir_suffix(scope_name: &str) -> String {
        format!("{TEST_SUBDIR_NAME_SUFFIX}-{scope_name}")
    }

    pub fn touch(path: &Path) -> Result<(), TestSetupError> {
        use std::fs::OpenOptions;

        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .map_err(|e| {
                format!(
                    "touching file under {}: {}",
                    path.to_str().unwrap_or("[p was invalid unicode]"),
                    e
                )
            })
            .map(|_| Ok(()))?
    }

    fn now_stamp(format: &str) -> String {
        use chrono::prelude::{DateTime, Utc};
        use std::time::SystemTime;
        let date_time: DateTime<Utc> = SystemTime::now().into();
        date_time.format(format).to_string()
    }
}
