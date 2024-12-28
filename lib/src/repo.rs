use std::convert::From;
use std::fmt;
use std::path::PathBuf;

// TODO needed? helpful?
pub type DirPath = PathBuf;

#[derive(Debug)]
pub enum RepoLoadError {
    /// A system-level error, not necessarily related to any VCS, eg: the directory doesn't exist,
    /// or we don't have access to it, etc.
    Directory(String),

    /// An unknown error ocurred trying to inspect the repo.
    Unknown(String),
}

impl From<String> for RepoLoadError {
    fn from(item: String) -> Self {
        RepoLoadError::Unknown(item)
    }
}

// TODO(rust) is this necessary? can this be derived by something for me?
impl fmt::Display for RepoLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            RepoLoadError::Unknown(msg) => write!(f, "{}", msg)?,
            RepoLoadError::Directory(msg) => write!(f, "{}", msg)?,
        }
        Ok(())
    }
}

// TODO is returning boolean right here? how can we handle the case that JJ repo is a JJ
// rpeo, or maybe a JJ-colocated-git repo, or JJ-colocated-p4 repo, or JJ-wrapping-git
// repo? Just true for all of those? Or some generic type we can define that would let JJ
// pack the answer here?
// TDOO design error return type that can distinguish between OS/access errors, and simply :
// no, this isn't a Self repo.
//fn is_vcs(dir: DirPath) -> Result<Option<Self>, &'static str>;

/// Operations any VCS should be able to answer about a repo.
// TODO finish convert from readme list to proper API surfaces/docs below (then update the
// readme to point here as the canonical reference).
pub trait Repo
where
    Self: std::fmt::Debug,
{
    /// Prints the root dir of the repo.
    fn root(&self) -> Result<DirPath, RepoLoadError>;
}
