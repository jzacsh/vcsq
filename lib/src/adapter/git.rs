use crate::repo::{DirPath, Driver, DriverError, Validator, VcsAvailable, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

static VCS_BIN_NAME: &str = "git";

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

#[derive(Debug)]
pub struct Loader
where
    Self: Sized;

impl Validator for Loader {
    /// Whether `dir` is a git repo (if so: wraps it in an object you can call for more
    /// questions.
    ///
    /// Basically checks the following shell command returns 0:
    /// ```sh
    /// ( cd "$1"; git rev-parse --show-toplevel >/dev/null 2>&1; )
    /// ```
    fn new_driver(&self, dir: DirPath) -> Result<Option<Box<dyn Driver>>, DriverError> {
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
            let repo: Box<dyn Driver> = Box::from(repo);
            Ok(Some(repo))
        } else {
            Ok(None)
        }
    }

    fn check_health(&self) -> Result<VcsAvailable, DriverError> {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.arg("--version");
        DriverError::expect_cmd_lossy("git cli: exec".to_string(), cmd.output())
    }
}

impl Repo {
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

    fn git_tracked_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        // TODO: (bug) investigate more, but manual testing shows --no-cached doesn't actually
        // work/change anything about ls-files behavior.
        cmd.arg("ls-files").arg("--no-cached");
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
        let files = lines
            .into_iter()
            .map(|ln| {
                // first 3 chars are modification-indicators like "?? " to indicate the file is
                // untracked.
                ln.chars().skip(3).collect::<String>()
            })
            .map(PathBuf::from)
            .collect();
        Ok(files)
    }

    fn tracked_files(&self) -> Result<Vec<DirPath>, DriverError> {
        let lines = DriverError::expect_cmd_lines(
            self.git_tracked_files().output(),
            0, /*min_lines*/
            "git cli: exec",
            None,
        )?;
        let files = lines.into_iter().map(PathBuf::from).collect();
        Ok(files)
    }
}
