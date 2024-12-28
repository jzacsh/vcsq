use crate::repo::{DirPath,Repo};
use std::process::{Command,Stdio};

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
    pub fn new(dir: DirPath) -> Result<Option<Self>, &'static str> {
        if Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            // TODO map stderr to Err() values
            .stderr(Stdio::null())
            // TODO check 'output.stdout' is a non-empty substr of 'dir'
            .stdout(Stdio::null())
            .current_dir(dir.clone())
            .output()
            .expect("failed executing git locally")
            .status
            .success() {
            Ok(Some(RepoGit {dir}))
        } else {
            Ok(None)
        }
    }
}

impl Repo for RepoGit {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO implement some "CmdRunner" interface so we can abstract out shelled-out commands.
        assert_eq!("git.rs -42", "git.rs 42");
    }
}
