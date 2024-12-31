use crate::adapter::git::RepoGit;
use crate::adapter::hg::RepoHg;
use crate::adapter::jj::RepoJj;
use crate::repo::{DirPath, Repo, RepoLoadError};

/// The particular brands of VCS this library supports.
#[derive(Debug, Clone)]
pub enum VcsBrand {
    Git,
    Mercurial,
    Jujutsu,
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
    pub fn new(dir: DirPath) -> Result<RepoPlexer, RepoLoadError> {
        let mut attempts = Vec::with_capacity(5);

        // TODO: (feature) generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ translate that to an
        // error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        attempts.push(VcsBrand::Git);
        if let Some(git) = RepoGit::new(dir.clone())? {
            return Ok(Self {
                brand: attempts.last().expect("bug: just pushed vcs enum").clone(),
                adapter: Box::from(git),
            });
        }

        attempts.push(VcsBrand::Mercurial);
        if let Some(hg) = RepoHg::new(dir.clone())? {
            return Ok(Self {
                brand: attempts.last().expect("bug: just pushed vcs enum").clone(),
                adapter: Box::from(hg),
            });
        }

        attempts.push(VcsBrand::Jujutsu);
        if let Some(jj) = RepoJj::new(dir.clone())? {
            return Ok(Self {
                brand: attempts.last().expect("bug: just pushed vcs enum").clone(),
                adapter: Box::from(jj),
            });
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

// NOTE: lack of unit tests here, is purely because of the coverage via e2e tests via ../vcst
// binary target. That doesn't mean unit tests won't be appropriate in this file in the future.
