use crate::cmd::{Utf8CmdOutput, Utf8CmdOutputLossy};
use std::convert::From;
use std::path::PathBuf;
use std::process::Output;
use thiserror::Error;

pub type DirPath = PathBuf;

pub const ERROR_REPO_NOT_DIRTY: &str = "repo not dirty";

#[derive(Error, Debug)]
pub enum RepoLoadError {
    /// A system-level error, not necessarily related to any VCS, eg: the directory doesn't exist,
    /// or we don't have access to it, etc.
    #[error("directory access issue: {0}")]
    Directory(String),

    /// An error ocurred trying to call out to the VCS binary
    #[error("vcs call failed: {:?}: {:?}", .context, .source)]
    Command {
        context: String,
        source: std::io::Error,
    },

    /// VCS binary failed and printed an error message
    #[error("vcs stderr: {:?}: {:?}", .context, .stderr)]
    Stderr { context: String, stderr: String },

    /// An error ocurred reading the directory name
    #[error("vcs returned a problematic root name")]
    RootName(#[from] std::string::FromUtf8Error),

    /// An unknown error ocurred
    #[error("{0}")]
    Unknown(String),
}

impl RepoLoadError {
    /// Low-level unwrapping of a command that's strict about its expectations that the
    /// underlying CLI produces valid utf8 cntent.
    ///
    /// For a lossy versin of this function see `unwrap_cmd_lossy(...)`.
    // TODO: (cleanup) switch all callers to strict handling: use {unwrap,expect}_cmd instead of
    // their lossy counterparts.
    pub fn unwrap_cmd(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutput, Self> {
        let output = cmd_output.map_err(|e| RepoLoadError::Command {
            context: context.clone(),
            source: e,
        })?;
        Ok(Utf8CmdOutput::from(output))
    }

    /// Like `unwrap_cmd(...)` but additionally expects the command to have succeeded, otherwise
    /// unpacks the stderr into an Err() case for you.
    ///
    /// For a lossy version of this function see `expect_cmd_lossy(...)`.
    pub fn expect_cmd(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutput, Self> {
        let utf8_output = Self::unwrap_cmd(context.clone(), cmd_output)?;
        if !utf8_output.status.success() {
            return Err(RepoLoadError::Stderr {
                context: context.clone(),
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

    /// Assumes cmd_output is an interaction with a textual CLI and does a dirty (lossy) conersion
    /// of its stdout/stderr outputs.
    ///
    /// For a strict conversion (where you want to handle bad UTF8-behaviors) see
    /// `unwrap_cmd(...)`.
    pub fn unwrap_cmd_lossy(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutputLossy, Self> {
        let output = cmd_output.map_err(|e| RepoLoadError::Command {
            context: context.clone(),
            source: e,
        })?;
        Ok(Utf8CmdOutputLossy::from(output))
    }

    /// Like `unwrap_cmd_lossy(...)` but additionally expects the command to have succeeded,
    /// otherwise unpacks the stderr into an Err() case for you.
    ///
    /// For a strict conversion (where you want to handle bad UTF8-behaviors) see
    /// `expect_cmd(...)`.
    pub fn expect_cmd_lossy(
        context: String,
        cmd_output: std::io::Result<Output>,
    ) -> Result<Utf8CmdOutputLossy, Self> {
        let utf8_output = Self::unwrap_cmd_lossy(context.clone(), cmd_output)?;
        if !utf8_output.status.success() {
            return Err(RepoLoadError::Stderr {
                context: context.clone(),
                stderr: utf8_output.stderr,
            });
        }
        Ok(utf8_output)
    }

    /// Like `expect_cmd_lossy(...)`  but adds the expectation that one stdout line will have been
    /// printed.
    ///
    // TODO: (rust) how to make this take _either_ (Utf8CmdOutputLossy, Utf8CmdOutput)? can we
    // reorganize one struct to be a subset of the other?
    // TODO: (codehealth) once the above TODO on type-cleanup is fixed, then redesign other APIs
    // above to be less all-in-one (they should accept the Utf8*Output* APIs, not generate them
    // internally).
    pub fn expect_cmd_line(context: String, output: Utf8CmdOutputLossy) -> Result<String, Self> {
        let lines = output.stdout_strings();
        if lines.len() > 1 {
            return Err(RepoLoadError::Unknown(format!(
                "unexpectedly got multiple ({}) lines: {}:\n'''\n{:?}\n'''\n'''",
                lines.len(),
                context,
                lines
            )));
        }
        Ok(lines
            .last()
            .ok_or_else(|| {
                RepoLoadError::Unknown(format!("unexpectedly returned empty output: {}", context))
            })?
            .to_string())
    }

    // TODO: (rust) idiomatic api is probably Iter<> of String, not Vec? try to fix that here
    pub fn expect_cmd_lines(
        context: String,
        output: Utf8CmdOutputLossy,
    ) -> Result<Vec<String>, Self> {
        let lines = output.stdout_strings();
        if lines.is_empty() {
            return Err(RepoLoadError::Unknown(format!(
                "{}: unexpectedly returned no lines",
                context
            )));
        }
        Ok(lines)
    }
}

impl From<String> for RepoLoadError {
    fn from(item: String) -> Self {
        RepoLoadError::Unknown(item)
    }
}

impl From<std::io::Error> for RepoLoadError {
    // TODO: (cleanup) findout if this is getting called anywhere, anad maybe delete/improve this
    // case
    fn from(source: std::io::Error) -> Self {
        RepoLoadError::Command {
            context: "bug: unexpected io error".to_string(),
            source,
        }
    }
}

/// Operations any VCS should be able to answer about a repo.
pub trait Repo
where
    Self: std::fmt::Debug,
{
    // TODO: is returning boolean/Option<> the right design here? wrt:
    //   ```rust
    //   fn new(dir) -> Result<Option<Repo>, ...> { ... }
    //   ```
    // that is: how can we handle the case that JJ repo is a JJ repo, or maybe a JJ-colocated-git repo,
    // or JJ-colocated-p4 repo, or JJ-wrapping-git repo? Just true for all of those? Or some generic
    // type we can define that would let JJ pack the answer here?

    // TODO: (rust) ability to provide an API for plexer.rs to use, so it knwos it always can call an
    // adapter's new() with the same api? ie:
    // ```rs
    //   fn new(dir: DirPath) -> Result<Option<Repo>, RepoLoadError>;
    // ```
    // Right now we do this by hand (trying to keep them in sync) but my attempts to describe this with
    // types has lead to fights against object-size knowledge rustc complains about.

    /// Prints the root dir of the repo.
    fn root(&self) -> Result<DirPath, RepoLoadError>;

    /// Lists filepaths touched that are the cause of the repo being dirty, or (assuming `clean_ok`) simply lists no output is
    /// the repo isn't dirty (thus can be used as a 1:1 proxy for IsClean's behavior).
    ///
    /// Should return an error if repo isn't dirty and not `clean_ok`
    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, RepoLoadError>;
}
