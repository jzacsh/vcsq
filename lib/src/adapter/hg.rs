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

        let is_ok = repo
            .hg_root()
            // TODO: (feature) check 'output.stdout' is a non-empty substr of 'dir'
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|e| RepoLoadError::Command {
                context: Some("hg cli"),
                source: e,
            })?
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
        let output = self.hg_root().output()?;
        if !output.status.success() {
            return Err("bug? silent error from hg".to_string().into());
        }
        let stdout = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(PathBuf::from(stdout))
    }

    fn dirty_files(&self) -> Result<Vec<DirPath>, RepoLoadError> {
        let output = self
            .dirty_files()
            .output()
            .map_err(|e| RepoLoadError::Command {
                context: Some("hg cli"),
                source: e,
            })?;
        if !output.status.success() {
            return Err(RepoLoadError::Stderr {
                context: Some("hg cli"),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .filter(|ln| !ln.is_empty())
            .map(PathBuf::from)
            .collect())
    }
}
