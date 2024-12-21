use crate::adapter::git::RepoGit;
use crate::repo::{DirPath, Repo};

/// The particular brands of VCS this library supports.
#[derive(Debug)]
enum VcsBrand {
    Git,
    // TODO: Mercurial,
    // TODO: Jujutsu,
}

#[derive(Debug)]
pub struct RepoPlexer {
    brand: VcsBrand,
    dir: DirPath,
}

/// Multiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking repo::Repo questions.
impl RepoPlexer {
    /// is dir `foo/` a VCS repo?
    /// if so, of which type?
    pub fn from(dir: DirPath) -> Result<RepoPlexer, &'static str> {
        todo!();
    }
}

impl Repo for RepoPlexer {
    /// Redundant: no point in calling this if you have an instance of RepoPlexer constructed
    fn is_vcs(dir: &DirPath) -> Result<bool, &str> {
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
