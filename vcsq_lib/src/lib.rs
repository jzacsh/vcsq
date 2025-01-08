//! # vcsq lib: Version Control System (VCS) Querying Library
//!
//! Couple wrapping traits to encompass simple questions you might ask of any common VCS,
//! regardless of its branding. Also some default implementations are exposed in this crate. The
//! intention is to make it easier to build tools that need to answer the simple questions about a
//! Version Control System (VCS) repository. For example, see `vcsq_cli`.
//!
//! Jargon used in this library:
//! - "brand" of VCS is some concrete repo format (eg: "Git" or "Mercurial" are brands).
//! - "driver" or "adapter" is an object that can answer queries about on particular brand.
//!   - eg: see the [`repo::Driver`] and [`repo::Validator`] traits that every adapter must
//!     implement.
//!
//! See vcsq repo for more: <https://gitlab.com/jzacsh/vcsq>
mod cmd;

/// Defines basics outline of a VCS repo and the queries vcsq is meant to handle.
pub mod repo;

/// Special pseudo-adapter of a repo that itself just demultiplexes all queries out to concrete
/// drivers (see [`adapter`] mod) of each brand of VCS that this library ships with support for.
pub mod plexer;

/// Concrete implementations of a handful of VCS brands.
// TODO: maybe surface via its own/separate crate if this lib ever gets to 1.0.0 (_before_ that).
pub mod adapter;
