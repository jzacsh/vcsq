use crate::repo::{DirPath, Driver, DriverError, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

impl Repo {
    pub fn new(dir: DirPath) -> Result<Option<Self>, DriverError> {
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
            Ok(Some(repo))
        } else {
            Ok(None)
        }
    }

    fn start_shellout(&self) -> Command {
        let mut cmd = Command::new("jj");
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
}

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        let output = DriverError::expect_cmd_lossy("jj cli".to_string(), self.jj_root().output())?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "jj cli".to_string(),
            output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.jj_dirty_files().output(),
            min_lines,
            "jj cli: exec".to_string(),
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines.into_iter().map(PathBuf::from).collect();
        Ok(dirty_files)
    }
}
