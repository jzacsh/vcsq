use crate::repo::{DirPath, Repo, RepoLoadError};
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

    fn dirty_files(&self) -> Command {
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

impl Repo for RepoHg {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output =
            RepoLoadError::expect_cmd_lossy("hg cli".to_string(), self.hg_root().output())?;
        output
            .stdout
            .lines()
            .last()
            .map(PathBuf::from)
            .ok_or_else(|| {
                RepoLoadError::Unknown("hg cli unexpectedly returned empty output".to_string())
            })
    }

    fn dirty_files(&self) -> Result<Vec<DirPath>, RepoLoadError> {
        let output =
            RepoLoadError::expect_cmd_lossy("hg cli".to_string(), self.dirty_files().output())?;
        Ok(output
            .stdout
            .lines()
            .filter(|ln| !ln.is_empty())
            .map(PathBuf::from)
            .collect())
    }
}
