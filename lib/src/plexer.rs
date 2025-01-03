use crate::adapter::git;
use crate::adapter::hg;
use crate::adapter::jj;
use crate::repo::{AncestorRef, DirPath, Driver, DriverError, RepoRef, RepoRefId, RepoRefName};

/// The particular brands of VCS this library supports.
#[derive(Debug, Clone)]
pub enum VcsBrand {
    Git,
    Mercurial,
    Jujutsu,
}

/// Multiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking `repo::Repo` questions.
#[derive(Debug)]
pub struct RepoPlexer {
    pub brand: VcsBrand,
    adapter: Box<dyn Driver>,
}

impl RepoPlexer {
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Repo object that can answer further questions about said repo.
    pub fn new(dir: DirPath) -> Result<RepoPlexer, DriverError> {
        let mut attempts = Vec::with_capacity(5);

        // TODO: (feature) generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ translate that to an
        // error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        attempts.push(VcsBrand::Git);
        if let Some(git) = git::Repo::new(dir.clone())? {
            return Ok(Self {
                brand: attempts.last().expect("bug: just pushed vcs enum").clone(),
                adapter: Box::from(git),
            });
        }

        attempts.push(VcsBrand::Mercurial);
        if let Some(hg) = hg::RepoHg::new(dir.clone())? {
            return Ok(Self {
                brand: attempts.last().expect("bug: just pushed vcs enum").clone(),
                adapter: Box::from(hg),
            });
        }

        attempts.push(VcsBrand::Jujutsu);
        if let Some(jj) = jj::RepoJj::new(dir.clone())? {
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

impl Driver for RepoPlexer {
    fn root(&self) -> Result<DirPath, DriverError> {
        self.adapter.root()
    }

    fn is_clean(&self) -> Result<bool, DriverError> {
        self.adapter.is_clean()
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        self.adapter.dirty_files(clean_ok)
    }

    fn parent_ref(&self) -> Result<RepoRef, DriverError> {
        self.adapter.parent_ref()
    }

    fn parent_ref_id(&self) -> Result<RepoRefId, DriverError> {
        self.adapter.parent_ref_id()
    }

    fn parent_ref_name(&self) -> Result<RepoRefName, DriverError> {
        self.adapter.parent_ref_name()
    }

    // TODO: (rust) wrt `limit`: there's a type-way to express positive natural numbers, yeah?
    fn first_ancestor_ref_name(&self, limit: Option<u64>) -> Result<AncestorRef, DriverError> {
        self.adapter.first_ancestor_ref_name(limit)
    }

    fn current_ref(&self, dirty_ok: bool) -> Result<RepoRef, DriverError> {
        self.adapter.current_ref(dirty_ok)
    }
    fn current_ref_id(&self, dirty_ok: bool) -> Result<RepoRefId, DriverError> {
        self.adapter.current_ref_id(dirty_ok)
    }
    fn current_ref_name(&self, dirty_ok: bool) -> Result<RepoRefName, DriverError> {
        self.adapter.current_ref_name(dirty_ok)
    }
}

// NOTE: lack of unit tests here, is purely because of the coverage via e2e tests via ../vcst
// binary target. That doesn't mean unit tests won't be appropriate in this file in the future.
