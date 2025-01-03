use crate::repo::{DirPath, Driver, RepoLoadError, ERROR_REPO_NOT_DIRTY};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct RepoHg {
    dir: DirPath,
}

impl RepoHg {
    pub fn new(dir: DirPath) -> Result<Option<Self>, RepoLoadError> {
        let repo = RepoHg { dir };

        let is_ok = RepoLoadError::unwrap_cmd_lossy(
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
        let mut cmd = Command::new("hg");
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

impl Driver for RepoHg {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output =
            RepoLoadError::expect_cmd_lossy("hg cli: exec".to_string(), self.hg_root().output())?;
        Ok(PathBuf::from(RepoLoadError::expect_cmd_line(
            "hg cli".to_string(),
            output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, RepoLoadError> {
        let min_lines = if clean_ok { 0 } else { 1 };
        let lines = RepoLoadError::expect_cmd_lines(
            self.hg_dirty_files().output(),
            min_lines,
            "hg cli: exec".to_string(),
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
