use crate::adapter::git;
use crate::adapter::hg;
use crate::adapter::jj;
use crate::repo;
use crate::repo::{AncestorRef, Driver, DriverError, QueryDir, Validator, VcsAvailable};
use std::num::NonZero;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

/// The particular brands of VCS this library supports.
#[derive(Debug, Clone, EnumIter, AsRefStr, PartialEq)]
pub enum VcsBrand {
    Git,
    Mercurial,
    Jujutsu,
}

/// Demultiplexes all available VCS adapters into one interface so you don't have to figure out which
/// VCS you're interacting with in order to start asking `repo::Repo` questions.
#[derive(Debug)]
pub struct Repo {
    pub brand: VcsBrand,
    adapter: Box<dyn Driver>,
}

impl Repo {
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Repo object that can answer further questions about said repo.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`] if either no VCS driver is present that recognizes the directory,
    /// or if some critical error happened (like one of the drivers hit an access error to the
    /// directory, or found something silly like the directory is actually a plain file).
    pub fn new_driver(dir: &QueryDir) -> Result<Self, DriverError> {
        // TODO: (feature) generically handle "vcs" being not in $PATH, out here in our plexer; if
        // _none_ of our adapter's underlying CLIs are in our plexer, _then_ translate that to an
        // error.
        //    if let NotFound = e.kind() { ... }
        //    https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound

        for current_attempt in VcsBrand::iter() {
            let loader: Box<dyn repo::Validator> = match &current_attempt {
                VcsBrand::Git => Box::from(git::Loader {}),
                VcsBrand::Mercurial => Box::from(hg::Loader {}),
                VcsBrand::Jujutsu => Box::from(jj::Loader {}),
            };
            if let Some(adapter) = loader.new_driver(dir.clone())? {
                return Ok(Self {
                    brand: current_attempt.clone(),
                    adapter,
                });
            }
        }

        Err(format!(
            "if dir is a VCS, it's of an unknown brand (tried these {:?}: {})",
            VcsBrand::iter().len(),
            VcsBrand::iter()
                .map(|b| b.as_ref().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
        .into())
    }
}

/// Basic report from a given brand of VCS (eg: `--version` output), intended purely to indicate
/// whether the VCS is in the current process's `$PATH`.
///
/// See also [`Validator.check_health`].
pub struct VcsHealth {
    /// Which specific brand of VCS we're reporting on.
    pub brand: VcsBrand,
    pub health: Result<VcsAvailable, DriverError>,
}

/// Returns all VCS drivers' health reports.
#[must_use]
pub fn check_health() -> Vec<VcsHealth> {
    VcsBrand::iter()
        .map(|brand| match brand {
            VcsBrand::Git => {
                let validator: Box<dyn Validator> = Box::from(git::Loader {});
                (brand, validator.check_health())
            }
            VcsBrand::Mercurial => {
                let validator: Box<dyn Validator> = Box::from(hg::Loader {});
                (brand, validator.check_health())
            }
            VcsBrand::Jujutsu => {
                let validator: Box<dyn Validator> = Box::from(jj::Loader {});
                (brand, validator.check_health())
            }
        })
        .map(|(brand, health)| VcsHealth { brand, health })
        .collect::<Vec<VcsHealth>>()
}

impl Driver for Repo {
    fn root(&self) -> Result<QueryDir, DriverError> {
        self.adapter.root()
    }

    fn is_clean(&self) -> Result<bool, DriverError> {
        self.adapter.is_clean()
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<QueryDir>, DriverError> {
        self.adapter.dirty_files(clean_ok)
    }

    fn tracked_files(&self) -> Result<Vec<QueryDir>, DriverError> {
        self.adapter.tracked_files()
    }

    fn parent_ref(&self) -> Result<repo::HistoryRef, DriverError> {
        self.adapter.parent_ref()
    }

    fn parent_ref_id(&self) -> Result<repo::HistoryRefId, DriverError> {
        self.adapter.parent_ref_id()
    }

    fn parent_ref_name(&self) -> Result<Option<repo::HistoryRefName>, DriverError> {
        self.adapter.parent_ref_name()
    }

    fn first_ancestor_ref_name(
        &self,
        limit: Option<NonZero<u64>>,
    ) -> Result<Option<AncestorRef>, DriverError> {
        self.adapter.first_ancestor_ref_name(limit)
    }

    fn current_ref(&self, dirty_ok: bool) -> Result<repo::HistoryRef, DriverError> {
        self.adapter.current_ref(dirty_ok)
    }
    fn current_ref_id(&self, dirty_ok: bool) -> Result<repo::HistoryRefId, DriverError> {
        self.adapter.current_ref_id(dirty_ok)
    }
    fn current_ref_name(
        &self,
        dirty_ok: bool,
    ) -> Result<Option<repo::HistoryRefName>, DriverError> {
        self.adapter.current_ref_name(dirty_ok)
    }
}

// NOTE: lack of unit tests here is purely because of the coverage via e2e tests via ../vcsq-cli/
// binary target. That doesn't mean unit tests won't be appropriate in this file in the future.
