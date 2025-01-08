//! Provides the traits any driver a particular brand of VCS must implement.
use crate::cmd::{Utf8CmdOutput, Utf8CmdOutputLossy};
use std::convert::From;
use std::path::PathBuf;
use std::process::Output;
use thiserror::Error;

/// The local repository a VCS query will center around.
pub type QueryDir = PathBuf;

pub const ERROR_REPO_NOT_CLEAN: &str = "repo not clean, references not hermetic";
pub const ERROR_REPO_NOT_DIRTY: &str = "repo not dirty";
pub const ERROR_REPO_NONEMPTY_OUTPUT: &str = "unexpectedly returned no lines";

#[derive(Error, Debug)]
pub enum DriverError {
    /// A system-level error, not necessarily related to any VCS, eg: the directory doesn't exist,
    /// or we don't have access to it, etc.
    #[error("directory access issue: {0}")]
    Directory(String),

    /// An error occurred trying to call out to the VCS binary
    #[error("vcs call failed: {:?}: {:?}", .context, .source)]
    Command {
        context: String,
        source: std::io::Error,
    },

    /// VCS binary failed and printed an error message
    #[error("vcs stderr: {:?}: {:?}", .context, .stderr)]
    Stderr { context: String, stderr: String },

    /// An error occurred reading the directory name
    #[error("vcs returned a problematic root name")]
    RootName(#[from] std::string::FromUtf8Error),

    /// An unknown error occurred
    #[error("{0}")]
    Unknown(String),
}

impl DriverError {
    /// Low-level unwrapping of a command that's strict about its expectations that the
    /// underlying CLI produces valid utf8 content.
    ///
    /// For a lossy version of this function see `unwrap_cmd_lossy(...)`.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] in the event of an underlying [`std::io::Error`].
    // TODO: (cleanup) switch all callers to strict handling: use {unwrap,expect}_cmd instead of
    // their lossy counterparts.
    pub fn unwrap_cmd(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutput, Self> {
        let output = cmd_output.map_err(|e| Self::Command { context, source: e })?;
        Ok(Utf8CmdOutput::from(output))
    }

    /// Like `unwrap_cmd(...)` but additionally expects the command to have succeeded, otherwise
    /// unpacks the stderr into an `Err()` case for you.
    ///
    /// For a lossy version of this function see `expect_cmd_lossy(...)`.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] in the event of an underlying [`std::io::Error`], or simply the
    /// stderr of `cmd_output` if the command actually exited non-zero.
    pub fn expect_cmd(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutput, Self> {
        let utf8_output = Self::unwrap_cmd(context.clone(), cmd_output)?;
        if !utf8_output.status.success() {
            return Err(Self::Stderr {
                context,
                stderr: utf8_output.stderr.map_err(|e| {
                    format!(
                        "bad utf8 from stderr: {}; lossy conversion: {}",
                        e, utf8_output.stderr_lossy
                    )
                })?,
            });
        }
        Ok(utf8_output)
    }

    /// Assumes `cmd_output` is an interaction with a textual CLI and does a dirty (lossy)
    /// conversion of its stdout/stderr outputs.
    ///
    /// For a strict conversion (where you want to handle bad UTF-8-behaviors) see
    /// `unwrap_cmd(...)`.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] in the event of an underlying [`std::io::Error`]. Does not check
    /// error state of the command though (see `expect_` variants of this API for that).
    pub fn unwrap_cmd_lossy(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutputLossy, Self> {
        let output = cmd_output.map_err(|e| Self::Command { context, source: e })?;
        Ok(Utf8CmdOutputLossy::from(output))
    }

    /// Like `unwrap_cmd_lossy(...)` but additionally expects the command to have succeeded,
    /// otherwise unpacks the stderr into an `Err()` case for you.
    ///
    /// For a strict conversion (where you want to handle bad UTF-8-behaviors) see
    /// `expect_cmd(...)`.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] in the event of an underlying [`std::io::Error`], or simply the
    /// stderr of `cmd_output` if the command actually exited non-zero.
    // TODO: rename 'expect' to either 'to'  or 'unwrap_ok_cmd_lossy'
    pub fn expect_cmd_lossy(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutputLossy, Self> {
        let utf8_output = Self::unwrap_cmd_lossy(context.clone(), cmd_output)?;
        if !utf8_output.status.success() {
            return Err(Self::Stderr {
                context,
                stderr: utf8_output.stderr,
            });
        }
        Ok(utf8_output)
    }

    /// Like `expect_cmd_lossy(...)`  but adds the expectation that one stdout line will have been
    /// printed.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if `output` didn't have exactly 1 line of stdout.
    // TODO: (rust) how to make this take _either_ (Utf8CmdOutputLossy, Utf8CmdOutput)? can we
    // reorganize one struct to be a subset of the other?
    // TODO: (codehealth) once the above TODO on type-cleanup is fixed, then redesign other APIs
    // above to be less all-in-one (they should accept the Utf8*Output* APIs, not generate them
    // internally).
    pub fn expect_cmd_line(context: &str, output: &Utf8CmdOutputLossy) -> Result<String, Self> {
        let lines = output.stdout_strings();
        if lines.len() > 1 {
            return Err(Self::Unknown(format!(
                "unexpectedly got multiple ({}) lines: {}:\n'''\n{:?}\n'''\n'''",
                lines.len(),
                context,
                lines
            )));
        }
        Ok(lines
            .last()
            .ok_or_else(|| Self::Unknown(format!("unexpectedly returned empty output: {context}")))?
            .to_string())
    }

    /// Like `expect_cmd_line(...)`  but might expect lines depending on `min_lines`, and doesn't
    /// care about the command's exit status.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if `output` had less than `min_lines` to stdout.
    // TODO: (rust) idiomatic API is probably Iter<> of String, not Vec? Try to fix that here
    pub fn expect_cmd_lines(
        output: std::io::Result<Output>,
        min_lines: u8,
        context: &str,
        expect_msg: Option<String>,
    ) -> Result<Vec<String>, Self> {
        let lines = Self::expect_cmd_lossy(context.to_string(), output)?.stdout_strings();
        if lines.len() < min_lines.into() {
            return Err(Self::Unknown(format!(
                "{}: {}",
                context,
                expect_msg.unwrap_or(ERROR_REPO_NONEMPTY_OUTPUT.to_string()),
            )));
        }
        Ok(lines)
    }
}

impl From<String> for DriverError {
    fn from(item: String) -> Self {
        DriverError::Unknown(item)
    }
}

/// VCS repo's canonical, machine-generated identifier describing a reference-point in its history
/// (eg: branch or tag in git, bookmark in jj).
///
/// These always exist, regardless of the point in history.
pub type HistoryRefId = String;

/// VCS repo's human-readable identifier describing a reference-point in its history (eg: branch or
/// tag in git, bookmark in jj).
///
/// These generally are sparse in a repo's history, unlike `HistoryRefId`.
pub type HistoryRefName = String;

/// Single point in time in the Repo's history.
pub struct HistoryRef {
    /// VCS's canonical identifier for this point in the repo's history.
    pub id: HistoryRefId,

    /// Hand-written, human-readable name of this point in history, if a human made one.
    pub name: Option<HistoryRefName>,

    /// Whether the repo was dirty when this result was generated (and therefore this isn't a
    /// hermetic description of the repo).
    pub dirty: bool,
}

pub struct AncestorRef {
    pub id: HistoryRefId,
    pub name: HistoryRefName,

    /// How far back of an ancestor is this (will always be 1 or more).
    // TODO: (rust) there's a type-way to express positive natural numbers, yeah?
    pub distance: u64,
}

pub type VcsAvailable = Utf8CmdOutputLossy;

/// Generic questions a VCS driver should be able to answer: is the VCS program even available on
/// this system? Does a directory even look like a valid VCS?.
pub trait Validator
where
    Self: std::fmt::Debug,
{
    /// Inspects on-disk directory path `dir` to determine if its a VCS repo, and if it is then
    /// returns a Driver that can answer further questions about said repo.
    ///
    /// # Errors
    ///
    /// Returns a [`DriverError`] if either this validator doesn't recognize the directory, as a
    /// VCS or if some critical error happened (like one of the drivers hit an access error to the
    /// directory, or found something silly like the directory is actually a plain file).
    fn new_driver(&self, dir: QueryDir) -> Result<Option<Box<dyn Driver>>, DriverError>;

    /// Returns basic info from the underlying VCS, proving its presence on the system, or an error
    /// if the attempt failed.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] in the event the underlying VCS APIs failed
    fn check_health(&self) -> Result<VcsAvailable, DriverError>;
}

/// Repo-specific questions any VCS should be able to answer.
///
/// Implementations of this trait can be expected from the factory `new_driver` of `Validator`
/// trait.
pub trait Driver
where
    Self: std::fmt::Debug,
{
    /// Prints the root dir of the repo.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn root(&self) -> Result<QueryDir, DriverError>;

    /// Whether repo is in a clean state.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn is_clean(&self) -> Result<bool, DriverError> {
        let dirty_files = self.dirty_files(true /*clean_ok*/)?;
        Ok(dirty_files.is_empty())
    }

    /// Lists filepaths touched that are the cause of the repo being dirty, or (assuming
    /// `clean_ok`) simply lists no output is the repo isn't dirty (thus can be used as a 1:1 proxy
    /// for `IsClean`'s behavior).
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed. Will also return this error if repo wasn't even dirty (unless `clean_ok`
    /// in which case an empty vector will be returned).
    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<QueryDir>, DriverError>;

    /// Lists filepaths tracked by this repo, ignoring the state of the repo edits (ie: any
    /// "staged" in git or deleted "working-copy" jj). The goal of this listing is to show the full
    /// listing of the repository's contents, as of the time of the current commit.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn tracked_files(&self) -> Result<Vec<QueryDir>, DriverError>;

    /// Returns the historical reference of the direct ancestor of the current state.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn parent_ref(&self) -> Result<HistoryRef, DriverError> {
        todo!(); // TODO: default implementation based on implementor's own impls of {parent_ref_id, parent_ref_name}
    }

    /// Thin wrapper for `parent_ref` that just unpacks the ID.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn parent_ref_id(&self) -> Result<HistoryRefId, DriverError> {
        todo!(); // TODO: (feature) delete and implement in adaapters
    }

    /// Thin wrapper for `parent_ref()` that just unpacks the name if there is one.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    fn parent_ref_name(&self) -> Result<Option<HistoryRefName>, DriverError> {
        todo!(); // TODO: (feature) delete and implement in adaapters
    }

    /// Walks up the ancestor history and returns the first encountered ref that has a
    /// human-made name. None return indicates a name doesn't exist on any of the ancestor refs, or
    /// none were seen before `limit` was steps-back were taken in history.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed.
    // TODO: (rust) wrt `limit`: there's a type-way to express positive natural numbers, yeah?
    fn first_ancestor_ref_name(
        &self,
        _limit: Option<u64>,
    ) -> Result<Option<AncestorRef>, DriverError> {
        todo!(); // TODO: (feature) delete and implement in adaapters
    }

    /// Returns the VCS ref for the current point in history.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed. Unless `dirty_ok`, then an error is returned when the repo is in a dirty
    /// state (as a reference for the current state is inherently not a reliable identifier).
    fn current_ref(&self, dirty_ok: bool) -> Result<HistoryRef, DriverError> {
        let is_dirty = !self.is_clean()?;
        if !dirty_ok && is_dirty {
            return Err(ERROR_REPO_NOT_CLEAN.to_string().into());
        }
        Ok(HistoryRef {
            id: self.current_ref_id(dirty_ok)?,
            name: self.current_ref_name(dirty_ok)?,
            dirty: is_dirty,
        })
    }

    /// Thin wrapper for `current_ref()` that just unpacks the name if there is one.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed. Unless `dirty_ok`, then an error is returned when the repo is in a dirty
    /// state (as a reference for the current state is inherently not a reliable identifier).
    fn current_ref_id(&self, _dirty_ok: bool) -> Result<HistoryRefId, DriverError>;

    /// Thin wrapper for `current_ref()` that just unpacks the name if there is one.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if (eg) there was a problem accessing the repo, or the underlying
    /// VCS APIs failed. Unless `dirty_ok`, then an error is returned when the repo is in a dirty
    /// state (as a reference for the current state is inherently not a reliable identifier).
    fn current_ref_name(&self, _dirty_ok: bool) -> Result<Option<HistoryRefName>, DriverError>;
}
