use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;
use thiserror::Error;

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

pub fn setup_temp_repos(tmpdir_root: &PathBuf) -> Result<(), TestSetupError> {
    setup_temp_repo_git(tmpdir_root.clone())?;
    setup_temp_repo_hg(tmpdir_root.clone())?;
    setup_temp_repo_jj(tmpdir_root.clone())?;
    Ok(())
}

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
    run_cli_from_tempdir("git", vec!["init", "test-git-repo"], tmpdir_root)
}

fn setup_temp_repo_hg(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
    run_cli_from_tempdir("hg", vec!["init", "test-hg-repo"], tmpdir_root)
}

fn setup_temp_repo_jj(tmpdir_root: PathBuf) -> Result<(), TestSetupError> {
    run_cli_from_tempdir(
        "jj",
        vec!["git", "init", "test-jj-on-git-repo"],
        tmpdir_root,
    )
}

/// How the hell is there no stdlib-esque functino for this??
pub fn mktemp(basename: &str) -> Result<PathBuf, TestSetupError> {
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

fn now_stamp(format: &str) -> String {
    use chrono::prelude::{DateTime, Utc};
    use std::time::SystemTime;
    let date_time: DateTime<Utc> = SystemTime::now().clone().into();
    date_time.format(format).to_string()
}
