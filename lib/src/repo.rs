use std::convert::From;
use std::path::PathBuf;
use thiserror::Error;

pub type DirPath = PathBuf;

#[derive(Error, Debug)]
pub enum RepoLoadError {
    /// A system-level error, not necessarily related to any VCS, eg: the directory doesn't exist,
    /// or we don't have access to it, etc.
    #[error("directory access issue: {0}")]
    Directory(String),

    /// An error ocurred trying to call out to the VCS binary
    #[error("vcs call failed: {:?}", .context)]
    Command {
        context: Option<&'static str>,
        source: std::io::Error,
    },

    /// An error ocurred reading the directory name
    #[error("vcs returned a problematic root name")]
    RootName(#[from] std::string::FromUtf8Error),

    /// An unknown error ocurred
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<String> for RepoLoadError {
    fn from(item: String) -> Self {
        RepoLoadError::Unknown(item)
    }
}

impl From<std::io::Error> for RepoLoadError {
    fn from(source: std::io::Error) -> Self {
        RepoLoadError::Command {
            context: None,
            source,
        }
    }
}

// TODO: is returning boolean right here? how can we handle the case that JJ repo is a JJ
// rpeo, or maybe a JJ-colocated-git repo, or JJ-colocated-p4 repo, or JJ-wrapping-git
// repo? Just true for all of those? Or some generic type we can define that would let JJ
// pack the answer here?

// TODO: (rust) ability to provide an API for plexer.rs to use, so it knwos it always can call an
// adapter's new() with the same api? ie:
// ```rs
//   fn new(dir: DirPath) -> Result<Option<Repo>, RepoLoadError>;
// ```
// Right now we do this by hand (trying to keep them in sync) but my attempts to describe this with
// types has lead to fights against object-size knowledge rustc complains about.

/// Operations any VCS should be able to answer about a repo.
pub trait Repo
where
    Self: std::fmt::Debug,
{
    /// Prints the root dir of the repo.
    fn root(&self) -> Result<DirPath, RepoLoadError>;
}
