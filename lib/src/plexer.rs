use crate::adapter::git::RepoGit;
use crate::repo::{DirPath, Repo};

/// The particular brands of VCS this library supports.
#[derive(Debug)]
enum VcsBrand {
    Git,
    // TODO: Mercurial,
    // TODO: Jujutsu,
}

/// Multiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking repo::Repo questions.
#[derive(Debug)]
pub struct RepoPlexer {
    brand: VcsBrand,
    adapter: Box<dyn Repo>,
    dir: DirPath,
}

// TODO the strange internal 'adapter: dyn Repo' causes a weird generic here, which then means that
// *telling rust* that RepoPlexer is _actually itself_ a Repo (which is what I want to do) would
// make for some really confsuing code. TODO here is to figure out how to sanely change this impl
// line from being
//    impl RepoPlexer<...> {}
// to instead being this (which we're doing by hand right now w/o compiler help):
//    impl Repo for RepoPlexer<...> {}
impl RepoPlexer {
    /// Redundant: no point in calling this if you have an instance of RepoPlexer constructed
    fn is_vcs(dir: DirPath) -> Result<Option<RepoPlexer>, &'static str> {
        if let Some(adapter) = RepoGit::is_vcs(dir.clone()).expect("error inspecting dir") {
            Ok(Some(Self {
                dir,
                brand: VcsBrand::Git,
                adapter: Box::from(adapter),
            }))
        } else {
            Err("if dir is a VCS, it's of an unknown brand")
        }
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
