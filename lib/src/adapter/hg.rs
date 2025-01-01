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
            RepoLoadError::expect_cmd_lossy("hg cli: exec".to_string(), self.hg_root().output())?;
        Ok(PathBuf::from(RepoLoadError::expect_cmd_line(
            "hg cli".to_string(),
            output,
        )?))
    }

    fn is_clean(&self) -> Result<bool, RepoLoadError> {
        todo!();
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, RepoLoadError> {
        let output = RepoLoadError::expect_cmd_lossy(
            "hg cli: exec".to_string(),
            self.dirty_files().output(),
        )?;
        let dirty_files = output.stdout_strings();
        if dirty_files.is_empty() {
            if clean_ok {
                return Ok(vec![]);
            }
            return Err(RepoLoadError::Unknown(format!(
                "hg cli: {}",
                ERROR_REPO_NOT_DIRTY
            )));
        }
        let dirty_files = dirty_files.into_iter().map(PathBuf::from).collect();
        Ok(dirty_files)
    }
}
