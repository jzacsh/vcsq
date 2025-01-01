use crate::repo::{DirPath, Repo, RepoLoadError, ERROR_REPO_NOT_DIRTY};
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

impl Repo for RepoHg {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output =
            RepoLoadError::expect_cmd_lossy("hg cli: exec".to_string(), self.hg_root().output())?;
        Ok(PathBuf::from(RepoLoadError::expect_cmd_line(
            "hg cli".to_string(),
            output,
        )?))
    }

    fn is_clean(&self) -> Result<bool, RepoLoadError> {
        RepoLoadError::is_clean(self)
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, RepoLoadError> {
        RepoLoadError::dirty_files(
            "hg cli: exec".to_string(),
            self.hg_dirty_files().output(),
            clean_ok,
        )
    }
}
