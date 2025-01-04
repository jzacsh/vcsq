use crate::repo::{DirPath, Driver, DriverError, Validator, VcsAvailable, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

static VCS_BIN_NAME: &str = "jj";

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

#[derive(Debug)]
pub struct Loader
where
    Self: Sized;

impl Validator for Loader {
    fn new_driver(&self, dir: DirPath) -> Result<Option<Box<dyn Driver>>, DriverError> {
        let repo = Repo { dir };

        let is_ok = DriverError::unwrap_cmd_lossy(
            "jj cli".to_string(),
            repo.jj_root()
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
        DriverError::expect_cmd_lossy("jj cli: exec".to_string(), cmd.output())
    }
}

impl Repo {
    fn start_shellout(&self) -> Command {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.current_dir(self.dir.clone());
        cmd
    }

    fn jj_root(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("root");
        cmd
    }

    fn jj_dirty_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("diff").arg("--name-only");
        cmd
    }

    fn jj_tracked_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        // TODO: unclear @- is always the right bet. _Sometimes_ you can put yourself into a weird
        // state where your '@' is actually not just an ephemeral copy, as you've re-attached
        // yourself to it.
        cmd.arg("file").arg("list").arg("-r").arg("@-");
        cmd
    }
}

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        let output = DriverError::expect_cmd_lossy("jj cli".to_string(), self.jj_root().output())?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "jj cli", &output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.jj_dirty_files().output(),
            min_lines,
            "jj cli: exec",
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines.into_iter().map(PathBuf::from).collect();
        Ok(dirty_files)
    }

    fn tracked_files(&self) -> Result<Vec<DirPath>, DriverError> {
        let lines = DriverError::expect_cmd_lines(
            self.jj_tracked_files().output(),
            0, /*min_lines*/
            "jj cli: exec",
            None,
        )?;
        let files = lines.into_iter().map(PathBuf::from).collect();
        Ok(files)
    }
}
