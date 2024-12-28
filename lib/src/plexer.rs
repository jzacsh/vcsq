use crate::adapter::git::RepoGit;
use crate::repo::{DirPath, Repo, RepoLoadError};

/// The particular brands of VCS this library supports.
#[derive(Debug)]
pub enum VcsBrand {
    Git,
    // TODO: Mercurial,
    // TODO: Jujutsu,
}

/// Multiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking repo::Repo questions.
#[derive(Debug)]
pub struct RepoPlexer {
    pub brand: VcsBrand,
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
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Repo object that can answer further questions about said repo.
    pub fn new(dir: DirPath) -> Result<Option<RepoPlexer>, RepoLoadError> {
        // TODO generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ trnaslate that
        // to an error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        let mut attempts = Vec::with_capacity(5);

        attempts.push(VcsBrand::Git);
        if let Some(git) = RepoGit::new(dir.clone()).expect("error inspecting dir") {
            Ok(Some(Self {
                dir,
                brand: VcsBrand::Git,
                adapter: Box::from(git),
            }))
        } else {
            Err(format!(
                "if dir is a VCS, it's of an unknown brand (tried {:?}: {:?})",
                attempts.len(),
                attempts).into())
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
