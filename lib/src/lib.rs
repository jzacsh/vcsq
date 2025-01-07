//! # vcsq-lib
//!
//! vcsq-lib makes it easy to build tools that need to answer questions about a Version Control
//! System (VCS) repository. For example, see vcsq-cli.
//!
//! Jargon used in this library:
//! - "brand" of VCS is some concrete repo format (eg: "git" or "merucurial" are brands).
//! - "driver" or "adapter" is an object that can answer queries about on particular brand.
//!   - eg: see the `repo::Driver` and `repo::Validator` traits that every adapter must implement.
//! - "driver" or "adapter" is an object that can answer queries about on particular brand.
//!
//! See vcsq repo for more: <https://gitlab.com/jzacsh/vcsq>
mod adapter;
mod cmd;

/// Defines basics outline of a VCS repo and the queries vcsq is meant to handle.
pub mod repo;

/// Special implementation of a repository that itself just de-multiplexes all queries out to
/// internal implementations of each brand of VCS that this library ships wth support for.
pub mod plexer;
