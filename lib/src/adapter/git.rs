use crate::repo::{DirPath, Driver, DriverError, Validator, VcsAvailable, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

static VCS_BIN_NAME: &str = "git";

#[derive(Debug)]
pub struct Loader;

impl Validator for Loader {
    fn check_health(&self) -> Result<VcsAvailable, DriverError> {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.arg("--version");
        DriverError::expect_cmd_lossy("git cli: exec".to_string(), cmd.output())
    }
}

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

impl Repo {
    /// Whether `dir` is a git repo (if so: wraps it in an object you can call for more
    /// questions.
    ///
    /// Basically checks the following shell command returns 0:
    /// ```sh
    /// ( cd "$1"; git rev-parse --show-toplevel >/dev/null 2>&1; )
    /// ```
    pub fn new(dir: DirPath) -> Result<Option<Self>, DriverError> {
        let repo = Repo { dir };
        let is_ok = DriverError::unwrap_cmd_lossy(
            "git cli".to_string(),
            repo.git_show_top_level()
                // TODO: (feature) check 'output.stdout' is a non-empty substr of 'dir'
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output(),
        )?
        .status
        .success();
        if is_ok {
            Ok(Some(repo))
        } else {
            Ok(None)
        }
    }

    fn start_shellout(&self) -> Command {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.current_dir(self.dir.clone());
        cmd
    }

    fn git_show_top_level(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("rev-parse").arg("--show-toplevel");
        cmd
    }

    fn git_dirty_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("status").arg("--porcelain");
        cmd
    }
}

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        let output = DriverError::expect_cmd_lossy(
            "git cli".to_string(),
            self.git_show_top_level().output(),
        )?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "git cli", &output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.git_dirty_files().output(),
            min_lines,
            "git cli: exec",
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines
            .into_iter()
            .map(|ln| {
                // first 3 chars are modification-indicators like "?? " to indicate the file is
                // untracked.
                ln.chars().skip(3).collect::<String>()
            })
            .map(PathBuf::from)
            .collect();
        Ok(dirty_files)
    }
}
