use crate::repo::{DirPath, Driver, DriverError, Validator, VcsAvailable, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Loader;

impl Validator for Loader {
    fn check_health(&self) -> Result<VcsAvailable, DriverError> {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.arg("--version");
        DriverError::expect_cmd_lossy("hg cli: exec".to_string(), cmd.output())
    }
}

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

static VCS_BIN_NAME: &str = "hg";

impl Repo {
    pub fn new(dir: DirPath) -> Result<Option<Self>, DriverError> {
        let repo = Repo { dir };

        let is_ok = DriverError::unwrap_cmd_lossy(
            "hg cli".to_string(),
            repo.hg_root()
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

    fn hg_root(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("root");
        cmd
    }

    fn hg_dirty_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.env("HGPLAIN", "1")
            .arg("status")
            .arg("--modified")
            .arg("--added")
            .arg("--removed")
            .arg("--deleted")
            .arg("--unknown");
        cmd
    }
}

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        let output =
            DriverError::expect_cmd_lossy("hg cli: exec".to_string(), self.hg_root().output())?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "hg cli", &output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.hg_dirty_files().output(),
            min_lines,
            "hg cli: exec",
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines
            .into_iter()
            .map(|ln| {
                // first 2 chars are modification-indicators like "?? " to indicate the file is
                // untracked.
                ln.chars().skip(2).collect::<String>()
            })
            .map(PathBuf::from)
            .collect();
        Ok(dirty_files)
    }
}
