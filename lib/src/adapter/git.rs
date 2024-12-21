use crate::repo;

#[derive(Debug)]
pub struct RepoGit {}

impl repo::Repo for RepoGit {
    fn is_vcs(dir: &repo::DirPath) -> Result<bool, &str> {
        // DO NOT SUBMIT use https://doc.rust-lang.org/std/process/struct.Command.html
        todo!(); // DO NOT SUBMIT: just shell out
    }
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
