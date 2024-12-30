use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

pub static TEST_VCS_BASENAME_GIT: &'static str = "test-git-repo";
pub static TEST_VCS_BASENAME_HG: &'static str = "test-hg-repo";
pub static TEST_VCS_BASENAME_JJ: &'static str = "test-jj-on-git-repo";
pub static TEST_VCS_BASENAME_NONVCS: &'static str = "test-not-vcs";
pub static TEST_VCS_BASENAME_NONDIR: &'static str = "test-not-dir";

// TODO: (rust) how much of this file do the following two crates make obsolete/deletable?
// - https://docs.rs/assert_cmd/latest/assert_cmd
// - https://docs.rs/predicates/latest/predicates

#[derive(Error, Debug)]
pub enum TestSetupError {
    #[error("system: {0}")]
    System(#[from] std::io::Error),

    #[error("cli failed and stderr had unicode problem: {0}")]
    CliFail(#[from] FromUtf8Error),

    #[error("vcs: {0}")]
    Command(String),
}

impl From<String> for TestSetupError {
    fn from(item: String) -> Self {
        TestSetupError::Command(item)
    }
}

#[derive(Debug)]
pub struct TestDirs {
    root_dir: PathBuf,
    pub git_repo: PathBuf,
    pub hg_repo: PathBuf,
    pub jj_repo: PathBuf,
    pub not_vcs: PathBuf,
    pub not_dir: PathBuf,
}

impl TestDirs {
    /// Reads from disk to find the latest temp directory tree.
    pub fn new(testdir_bname: &str) -> Result<Self, TestSetupError> {
        let root_dir = Self::list_temp_repos(testdir_bname)?;
        let mut git_repo = root_dir.clone();
        git_repo.push(TEST_VCS_BASENAME_GIT);
        let mut hg_repo = root_dir.clone();
        hg_repo.push(TEST_VCS_BASENAME_HG);
        let mut jj_repo = root_dir.clone();
        jj_repo.push(TEST_VCS_BASENAME_JJ);

        let mut not_vcs = root_dir.clone();
        not_vcs.push(TEST_VCS_BASENAME_NONVCS);

        let mut not_dir = root_dir.clone();
        not_dir.push(TEST_VCS_BASENAME_NONDIR);

        not_dir /* DO NOT SUBMIT - do this for all of the above cases */
            .try_exists()
            .map_err(|e| format!("not_dir try_exists;{}", e))?;

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
    pub fn create(testdir_bname: &PathBuf) -> Result<(), TestSetupError> {
        use crate::common::vcs_test_setup::setup_temp_repos;
        setup_temp_repos(testdir_bname)
    }

    /// Finds the latest tempdir on disk given setup_temp_repos() was called with `testdir_bname`
    fn list_temp_repos(testdir_bname: &str) -> Result<PathBuf, TestSetupError> {
        use std::fs;

        let generic_root = make_test_temp::get_mktemp_root(testdir_bname)?;
        let test_run_dirs: Vec<PathBuf> = fs::read_dir(generic_root)?
            .into_iter()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<_, std::io::Error>>()?;
        let mut test_run_dirs: Vec<PathBuf> = test_run_dirs
            .into_iter()
            .filter(|p| p.is_dir())
            .collect::<Vec<_>>();
        test_run_dirs.sort();
        let newest_test_dir = test_run_dirs
            .into_iter()
            .last()
            .ok_or("no dirs found".to_string())?;
        Ok(newest_test_dir)
    }

    /// Generate a filepath we're pretty sure weon't exist (but in case some SUT tries to touch
    /// it, contain its path within TestDirs).
    pub fn non_extant(&self) -> PathBuf {
        use uuid::Uuid;
        let mut path = self.root_dir.clone();
        let uuidv4 = Uuid::new_v4();
        path.push(uuidv4.simple().to_string());
        assert!(
            !path.exists(),
            "tried to gen rando flie, but got real one: {:?}",
            path
        );
        path
    }
}

pub mod vcs_test_setup {
    use crate::common::TestSetupError;
    use crate::common::{
        TEST_VCS_BASENAME_GIT, TEST_VCS_BASENAME_HG, TEST_VCS_BASENAME_JJ,
        TEST_VCS_BASENAME_NONDIR, TEST_VCS_BASENAME_NONVCS,
    };
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    fn run_cli_from_tempdir(
        cmd: &str,
        args: Vec<&str>,
        tmpdir_root: PathBuf,
    ) -> Result<(), TestSetupError> {
        let cli_output = Command::new(cmd)
            .args(args)
            .stdout(Stdio::null())
            .current_dir(tmpdir_root)
            .output()?;
        if cli_output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8(cli_output.stderr)?.trim().to_string();
            Err(stderr.into())
        }
    }

    fn setup_temp_repo_git(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("git", vec!["init", TEST_VCS_BASENAME_GIT], tmpdir_root)
    }

    fn setup_temp_repo_hg(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("hg", vec!["init", TEST_VCS_BASENAME_HG], tmpdir_root)
    }

    fn setup_temp_repo_jj(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        run_cli_from_tempdir("jj", vec!["git", "init", TEST_VCS_BASENAME_JJ], tmpdir_root)
    }

    fn setup_temp_nonvcs_dir(mut tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        use std::fs::create_dir;
        tmpdir_root.push(TEST_VCS_BASENAME_NONVCS);
        create_dir(tmpdir_root)?;
        Ok(())
    }

    fn setup_temp_plainfile(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
        use crate::common::make_test_temp::touch;

        let mut plain_file = tmpdir_root.clone();
        plain_file.push(TEST_VCS_BASENAME_NONDIR);
        touch(plain_file.as_path())
    }
    /// Creates new temp directories on disk.
    pub fn setup_temp_repos(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
        setup_temp_repo_git(tmpdir_root.clone())?;
        setup_temp_repo_hg(tmpdir_root.clone())?;
        setup_temp_repo_jj(tmpdir_root.clone())?;
        setup_temp_nonvcs_dir(tmpdir_root.clone())?;
        setup_temp_plainfile(tmpdir_root.clone())?;
        Ok(())
    }
}

pub mod make_test_temp {
    use crate::common::TestSetupError;
    use std::path::{Path, PathBuf};

    pub fn get_mktemp_root(basename: &str) -> Result<PathBuf, TestSetupError> {
        use env::VarError;
        use std::env;
        use std::fs::create_dir;

        let mut root_dir: PathBuf = match env::var("VCST_TESTDIR") {
            Ok(d) => PathBuf::from(d),
            Err(e) => match e {
                VarError::NotUnicode(s) => panic!("VCST_TESTDIR had non-unicode value: {:?}", s),
                VarError::NotPresent => env::temp_dir(),
            },
        };
        assert!(
            root_dir.exists(),
            "expect temp root_dir exists: {:?}",
            root_dir
        );
        assert!(
            root_dir.is_dir(),
            "expect temp root_dir is dir: {:?}",
            root_dir
        );
        assert!(
            root_dir.to_str().expect("bad unicode in root_dir") != "/",
            "expect temp root_dir is not root: {:?}",
            root_dir
        );

        root_dir.push(basename);
        if !root_dir.exists() {
            create_dir(&root_dir)?
        }
        Ok(root_dir)
    }

    /// How the hell is there no stdlib-esque functino for this??
    pub fn mktemp(basename: &str) -> Result<PathBuf, TestSetupError> {
        use std::fs::create_dir;

        let mut root_dir: PathBuf = get_mktemp_root(basename)?;

        let iso_str = now_stamp("%F-at-%s");
        root_dir.push(iso_str);
        create_dir(&root_dir)?;

        assert!(
            root_dir.exists(),
            "bug: root_dir doesn't exist after creation: {:?}",
            root_dir
        );
        Ok(root_dir)
    }

    pub fn touch(path: &Path) -> Result<(), TestSetupError> {
        use std::fs::OpenOptions;

        OpenOptions::new()
            .create(true)
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
        let date_time: DateTime<Utc> = SystemTime::now().clone().into();
        date_time.format(format).to_string()
    }
}
