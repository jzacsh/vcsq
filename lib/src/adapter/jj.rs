use crate::repo::{DirPath, Repo, RepoLoadError};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct RepoJj {
    dir: DirPath,
}

impl RepoJj {
    pub fn new(dir: DirPath) -> Result<Option<Self>, RepoLoadError> {
        let repo = RepoJj { dir };

        let is_ok = repo
            .jj_root()
            // TODO: (feature) check 'output.stdout' is a non-empty substr of 'dir'
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|e| RepoLoadError::Command {
                context: Some("jj cli"),
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

    fn jj_root(&self) -> Command {
        let mut cmd = Command::new("jj");

        cmd.arg("root").current_dir(self.dir.clone());

        cmd
    }
}

impl Repo for RepoJj {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output = self.jj_root().output()?;
        if !output.status.success() {
            return Err("bug? silent error from jj".to_string().into());
        }
        let stdout = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(PathBuf::from(stdout))
    }

    fn dirty_files(&self) -> Result<Vec<DirPath>, RepoLoadError> {
        todo!();
    }
}
