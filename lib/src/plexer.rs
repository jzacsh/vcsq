use crate::adapter::git;
use crate::adapter::hg;
use crate::adapter::jj;
use crate::repo;
use crate::repo::{AncestorRef, DirPath, Driver, DriverError};

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
pub struct Repo {
    pub brand: VcsBrand,
    adapter: Box<dyn Driver>,
}

impl Repo {
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Repo object that can answer further questions about said repo.
    pub fn new(dir: DirPath) -> Result<Self, DriverError> {
        let mut attempts = Vec::with_capacity(5);

        // TODO: (feature) generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ translate that to an
        // error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        let current_attempt = VcsBrand::Git;
        attempts.push(current_attempt.clone());
        if let Some(git) = git::Repo::new(dir.clone())? {
            return Ok(Self {
                brand: current_attempt,
                adapter: Box::from(git),
            });
        }

        let current_attempt = VcsBrand::Mercurial;
        attempts.push(current_attempt.clone());
        if let Some(hg) = hg::Repo::new(dir.clone())? {
            return Ok(Self {
                brand: current_attempt,
                adapter: Box::from(hg),
            });
        }

        let current_attempt = VcsBrand::Jujutsu;
        attempts.push(current_attempt.clone());
        if let Some(jj) = jj::Repo::new(dir.clone())? {
            return Ok(Self {
                brand: current_attempt,
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

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        self.adapter.root()
    }

    fn is_clean(&self) -> Result<bool, DriverError> {
        self.adapter.is_clean()
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        self.adapter.dirty_files(clean_ok)
    }

    fn parent_ref(&self) -> Result<repo::HistoryRef, DriverError> {
        self.adapter.parent_ref()
    }

    fn parent_ref_id(&self) -> Result<repo::HistoryRefId, DriverError> {
        self.adapter.parent_ref_id()
    }

    fn parent_ref_name(&self) -> Result<repo::HistoryRefName, DriverError> {
        self.adapter.parent_ref_name()
    }

    // TODO: (rust) wrt `limit`: there's a type-way to express positive natural numbers, yeah?
    fn first_ancestor_ref_name(&self, limit: Option<u64>) -> Result<AncestorRef, DriverError> {
        self.adapter.first_ancestor_ref_name(limit)
    }

    fn current_ref(&self, dirty_ok: bool) -> Result<repo::HistoryRef, DriverError> {
        self.adapter.current_ref(dirty_ok)
    }
    fn current_ref_id(&self, dirty_ok: bool) -> Result<repo::HistoryRefId, DriverError> {
        self.adapter.current_ref_id(dirty_ok)
    }
    fn current_ref_name(&self, dirty_ok: bool) -> Result<repo::HistoryRefName, DriverError> {
        self.adapter.current_ref_name(dirty_ok)
    }
}

// NOTE: lack of unit tests here, is purely because of the coverage via e2e tests via ../vcst
// binary target. That doesn't mean unit tests won't be appropriate in this file in the future.
