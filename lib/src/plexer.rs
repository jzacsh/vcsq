use crate::adapter::git::RepoGit;
use crate::repo;

/// The particular brands of VCS this library supports.
enum VcsBrand {
    Git,
    // TODO: Mercurial,
    // TODO: Jujutsu,
}

pub struct RepoPlexer {}

impl repo::Repo for RepoPlexer {
    fn is_vcs(dir: &repo::DirPath) -> Result<bool, &str> {
        todo!(); // DO NOT SUBMIT: just shell out
    }
}

/// is dir `foo/` a VCS repo?
/// if so, of which type?
pub fn vcs_type(dir: &repo::DirPath) -> Result<VcsBrand, &str> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO implement some "CmdRunner" interface so we can abstract out shelled-out commands.
        assert_eq!("plexer -42", "plexer.rs 42");
    }
}
