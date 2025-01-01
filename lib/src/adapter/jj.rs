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

        let is_ok = RepoLoadError::unwrap_cmd_lossy(
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
}

impl Repo for RepoJj {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output =
            RepoLoadError::expect_cmd_lossy("jj cli".to_string(), self.jj_root().output())?;
        Ok(PathBuf::from(RepoLoadError::expect_cmd_line(
            "jj cli".to_string(),
            output,
        )?))
    }

    fn is_clean(&self) -> Result<bool, RepoLoadError> {
        todo!();
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, RepoLoadError> {
        todo!();
    }
}
