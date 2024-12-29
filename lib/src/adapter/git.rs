use crate::repo::{DirPath, Repo, RepoLoadError};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct RepoGit {
    dir: DirPath,
}

impl RepoGit {
    /// Whether `dir` is a git repo (if so: wraps it in an object you can call for more
    /// questions.
    ///
    /// Basically checks the following shell command returns 0:
    /// ```sh
    /// ( cd "$1"; git rev-parse --show-toplevel >/dev/null 2>&1; )
    /// ```
    pub fn new(dir: DirPath) -> Result<Option<Self>, RepoLoadError> {
        let repo_git = RepoGit { dir };
        let is_ok = repo_git
            .git_show_top_level()
            // TODO: (feature) check 'output.stdout' is a non-empty substr of 'dir'
            .stdout(Stdio::null())
            // TODO: map stderr to Err() values
            .stderr(Stdio::null())
            .output()
            .expect("failed executing git locally")
            .status
            .success();
        if is_ok {
            Ok(Some(repo_git))
        } else {
            Ok(None)
        }
    }

    fn git_show_top_level(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.arg("rev-parse")
            .arg("--show-toplevel")
            .current_dir(self.dir.clone());
        cmd
    }
}

impl Repo for RepoGit {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        let output = self.git_show_top_level().output()?;
        if !output.status.success() {
            return Err("bug? silent error from git".to_string().into());
        }
        let stdout = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(PathBuf::from(stdout))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!("git.rs -42", "git.rs 42");
    }
}
