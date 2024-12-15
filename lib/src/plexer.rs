use crate::adapter::git::RepoGit;
use crate::repo;

/// The particular brands of VCS this library supports.
enum VcsBrand {
    Git,
    // TODO: Mercurial,
    // TODO: Jujutsu,
}

pub struct RepoPlexer<'a> {
    brand: VcsBrand,
    dir: &'a repo::DirPath,
}

impl RepoPlexer<'_> {
    /// is dir `foo/` a VCS repo?
    /// if so, of which type?
    fn from(dir: &repo::DirPath) -> Result<RepoPlexer<'_>, &str> {
        todo!();
    }
}

impl repo::Repo for RepoPlexer<'_> {
    /// Redundant: no point in calling this if you have an instance of RepoPlexer constructed
    fn is_vcs(dir: &repo::DirPath) -> Result<bool, &str> {
        todo!(); // DO NOT SUBMIT: just shell out
    }
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
