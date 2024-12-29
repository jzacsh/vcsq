use crate::adapter::git::RepoGit;
use crate::adapter::hg::RepoHg;
use crate::repo::{DirPath, Repo, RepoLoadError};

/// The particular brands of VCS this library supports.
#[derive(Debug)]
pub enum VcsBrand {
    Git,
    Mercurial,
    // TODO::(feature) Jujutsu,
}

/// Multiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking repo::Repo questions.
#[derive(Debug)]
pub struct RepoPlexer {
    pub brand: VcsBrand,
    adapter: Box<dyn Repo>,
}

impl RepoPlexer {
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Repo object that can answer further questions about said repo.
    pub fn new(dir: DirPath) -> Result<Option<RepoPlexer>, RepoLoadError> {
        let mut attempts = Vec::with_capacity(5);

        // TODO: (feature) generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ translate that to an
        // error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        attempts.push(VcsBrand::Git);
        // TODO: (rust) stop panicking on every attempt, just handle the error appropriatley
        if let Some(git) = RepoGit::new(dir.clone()).expect("git error inspecting dir") {
            return Ok(Some(Self {
                brand: VcsBrand::Git,
                adapter: Box::from(git),
            }));
        }

        attempts.push(VcsBrand::Mercurial);
        // TODO: (rust) stop panicking on every attempt, just handle the error appropriatley
        if let Some(hg) = RepoHg::new(dir.clone()).expect("hg error inspecting dir") {
            return Ok(Some(Self {
                brand: VcsBrand::Mercurial,
                adapter: Box::from(hg),
            }));
        }

        Err(format!(
            "if dir is a VCS, it's of an unknown brand (tried {:?}: {:?})",
            attempts.len(),
            attempts
        )
        .into())
    }
}

impl Repo for RepoPlexer {
    fn root(&self) -> Result<DirPath, RepoLoadError> {
        self.adapter.root()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // TODO: (rust) decide on unit testing strategy here (how to dependency-inject filesystem
        // interactions? don't bother, just straight to e2e tests?)
        assert_eq!("plexer -42", "plexer.rs 42");
    }
}
