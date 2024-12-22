use crate::repo::{DirPath,Repo};

#[derive(Debug)]
pub struct RepoGit {}

impl RepoGit {
    /// Whether `dir` is a git repo (if so: wraps it in an object you can call for more
    /// questions.
    ///
    /// Basically checks the following shell command returns 0:
    /// ```sh
    /// ( cd "$1"; git rev-parse --show-toplevel >/dev/null 2>&1; )
    /// ```
    pub fn is_vcs(dir: DirPath) -> Result<Option<Self>, &'static str> {
        todo!()
        // DO NOT SUBMIT use https://doc.rust-lang.org/std/process/struct.Command.html
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
