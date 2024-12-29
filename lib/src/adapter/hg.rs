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
            // TODO check 'output.stdout' is a non-empty substr of 'dir'
            .stdout(Stdio::null())
            // TODO map stderr to Err() values
            .stderr(Stdio::null())
            .output()
            .expect("failed executing hg locally")
            .status
            .success();
        if is_ok {
            Ok(Some(repo))
        } else {
            Ok(None)
        }
    }

    fn hg_root(&self) -> Command {
        let mut cmd = Command::new("hg");

        cmd.arg("root").current_dir(self.dir.clone());

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
}
