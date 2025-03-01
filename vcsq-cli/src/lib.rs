//! `vcsq` CLI thinly wraps [`vcsq_lib`] to answer version-control questions about any directory.
//!
//! Usage: `vcsq SUB_CMD DIR`
//! Example: `vcsq is-clean .`
//!
//! See `--help` for complete doc, and README at <https://gitlab.com/jzacsh/vcsq> for more.
use clap::{Parser, Subcommand};
use std::io;
use std::num::NonZero;
use thiserror::Error;
use vcsq_lib::plexer;
use vcsq_lib::repo::{Driver, DriverError, QueryDir};

/// Top-Level instructions into which we parse the resluts of CLI args.
#[derive(Parser, Debug)]
#[command(
    name = "vcsq",
    version,
    about = "vcs queries in rust",
    long_about = "vcsq is a rust CLI providing Version Control System (VCS) inspection, without you
needing to know each VCS's proprietary incantations."
)]
pub struct MainArgs {
    /// Directory for which you'd like to ask VCS questions.
    #[arg(short, long)]
    pub dir: Option<QueryDir>,

    #[command(subcommand)]
    pub query: Option<QueryCmd>,
}

#[derive(Error, Debug)]
enum CliError {
    #[error("usage error: {0}")]
    Usage(String),

    #[error("vcs error: {0}")]
    Plexing(#[from] DriverError),

    #[error("{0}")]
    Unknown(String),
}

impl MainArgs {
    /// Alternative to clap's parse, just so we can handle defaults
    ///
    // TODO: (feature,clap) fix this clunkiness: somehow allow lone positional arg of a
    // directory for the case that no subcommand is passed. IDK how to do that in clap.
    pub(self) fn reduce(&self) -> Result<QueryCmd, CliError> {
        if let Some(q) = &self.query {
            Ok(q.clone())
        } else {
            let dir = self
                .dir
                .clone()
                .ok_or(CliError::Usage(
                    "require either subcmd with a query or a direct --dir".into(),
                ))?
                .clone();
            Ok(QueryCmd::Brand { dir })
        }
    }
}

/// Sub-commands of the CLI that map to a single VCS query.
// TODO: (clap, rust) figure out how to shorten the names of these subcommands so they rust-lang
// naming doesn't turn into annoyingly-long (and hyphenated) names.
//
// TODO: (feature) impl a subcommand that lets you know which $PATH dependencies are found.
#[derive(Debug, Subcommand, Clone)]
pub enum QueryCmd {
    /// Prints the brand of the VCS repo, or exits non-zero if it's not a known VCS repo.
    #[command(arg_required_else_help = true)]
    Brand { dir: QueryDir },

    /// Prints the root dir of the repo
    #[command(arg_required_else_help = true)]
    Root { dir: QueryDir },

    /// Whether VCS repo is in a clean state, or has uncommitted work.
    #[command(arg_required_else_help = true)]
    IsClean {
        // TODO: (feature) implement subcommand here, eg: enum {diffstat, diff, files}
        dir: QueryDir,
    },

    /// Print the VCS repo's current revision ID (eg: rev in Mercurial, ref in git, etc).
    #[command(arg_required_else_help = true)]
    CurrentId {
        dir: QueryDir,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        #[arg(long, default_value_t = false)]
        dirty_ok: bool,
    },

    /// Print the VCS repo's current human-readable revision (eg: branch or tag in git, bookmark in
    /// jj)
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    CurrentName {
        dir: QueryDir,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        #[arg(long, default_value_t = false)]
        dirty_ok: bool,
    },

    /// Print the VCS repo's parent revision ID to the current point in history (eg: rev in
    /// Mercurial, ref in git, etc).
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ParentId { dir: QueryDir },

    /// Print the VCS repo's parent revision's human-readable revision name for the first parent it
    /// finds with one, or until it has stepped --max steps. Non-zero exit with no stderr output
    /// indicates one wasn't found.
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ParentName {
        dir: QueryDir,

        /// Max number of parents back to walk when seeking a parent with a hand-written ref name.
        max: NonZero<u64>,
    },

    /// Print the VCS repo's descendent revision IDs, if any exit.
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ChildIds {
        dir: QueryDir,

        /// Max number of children to list, or zero to indicate no max.
        #[arg(long, default_value_t = 0)]
        max: u64,
    },

    /// Print the VCS repo's descendent revision ID, or error if more than one exists.
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ChildId { dir: QueryDir },

    /// Lists filepaths tracked by this repo, ignoring the state of the repo (ie: any "staged"
    /// (git) or deleted "working-copy" (jj) edits. The goal of this listing is to show the full
    /// listing of the repository's contents, as of the time of the current commit.
    #[command(arg_required_else_help = true)]
    TrackedFiles { dir: QueryDir },

    /// Lists filepaths touched that are the cause of the repo being dirty, or lists no output if
    /// the repo isn't dirty (thus can be used as a 1:1 proxy for `IsClean`'s behavior).
    #[command(arg_required_else_help = true)]
    DirtyFiles {
        dir: QueryDir,
        #[arg(long, default_value_t = false)]
        clean_ok: bool,
        // TODO: (feature) add flag like "--exists" to only show files that are currently present
        // (eg: so this can be piped right to an editor's args).
    },

    /// Prints what files were touched by the `CurrentId`
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    CurrentFiles {
        dir: QueryDir,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        #[arg(long, default_value_t = false)]
        dirty_ok: bool,
        // TODO: (feature) allow an optional Id or Name  (ref or bookmark) of which to compare
        // (instead of just the default which is "parent commit").

        // TODO: (feature) implement subcommand here, eg: enum {diffstat, diff, files} (unified
        // with IsClean)
    },

    /// Prints any system/$PATH info that might be useful for debugging issues this binary might
    /// have on your system.
    CheckHealth,
}

impl QueryCmd {
    fn dir(&self) -> Option<QueryDir> {
        self.dir_path().cloned()
    }

    // TODO: (rust) way to ask clap to make a global positional arg for all these subcommands, so
    // we can rely on its presence?
    fn dir_path(&self) -> Option<&QueryDir> {
        match self {
            QueryCmd::Brand { dir }
            | QueryCmd::Root { dir }
            | QueryCmd::IsClean { dir }
            | QueryCmd::DirtyFiles { dir, clean_ok: _ }
            | QueryCmd::TrackedFiles { dir }
            | QueryCmd::CurrentId { dir, dirty_ok: _ } => Some(dir),
            QueryCmd::CheckHealth => None,
            #[cfg(debug_assertions)]
            QueryCmd::CurrentName { dir, dirty_ok: _ }
            | QueryCmd::ParentId { dir }
            | QueryCmd::ParentName { dir, max: _ }
            | QueryCmd::ChildIds { dir, max: _ }
            | QueryCmd::ChildId { dir }
            | QueryCmd::CurrentFiles { dir, dirty_ok: _ } => Some(dir),
        }
    }
}

struct PlexerQuery<'a> {
    plexer: plexer::Repo,
    cli: QueryCmd,
    stdout: &'a mut dyn io::Write,
}

impl<'a> PlexerQuery<'a> {
    fn new(
        args: &'a MainArgs,
        stdout: &'a mut dyn io::Write,
    ) -> Result<Option<PlexerQuery<'a>>, CliError> {
        let query = args.reduce()?;
        let Some(dir) = query.dir() else {
            return Ok(None);
        };
        if !dir.is_dir() {
            return Err(CliError::Usage(
                "dir must be a readable directory".to_string(),
            ));
        }
        let plexer = plexer::Repo::new_driver(&dir)?;
        Ok(Some(PlexerQuery {
            plexer,
            cli: query,
            stdout,
        }))
    }

    pub fn handle_query(&mut self) -> Result<u8, CliError> {
        match self.cli {
            QueryCmd::Brand { dir: _ } => {
                writeln!(self.stdout, "{:?}", self.plexer.brand)
                    .unwrap_or_else(|_| panic!("failed stdout write of: {:?}", self.plexer.brand));
            }
            QueryCmd::Root { dir: _ } => {
                let root_path = self.plexer.root()?;
                let dir_path = root_path.as_path().to_str().ok_or_else(|| {
                    CliError::Unknown(format!("vcs generated invalid unicode: {root_path:?}"))
                })?;
                writeln!(self.stdout, "{dir_path}")
                    .unwrap_or_else(|_| panic!("failed stdout write of: {dir_path}"));
            }
            QueryCmd::IsClean { dir: _ } => {
                let is_clean = self.plexer.is_clean().map_err(CliError::Plexing)?;
                return Ok(u8::from(!is_clean));
            }
            QueryCmd::CheckHealth => panic!("bug: PlexerQuery() should not be constructed for the generalized CheckHealth query"),
            QueryCmd::CurrentId {
                dir: _,
                dirty_ok,
            } => {
                let current_id = self.plexer.current_ref_id(dirty_ok)?;
                writeln!(self.stdout, "{current_id}").unwrap_or_else(|_| {
                    panic!("failed stdout write of: {current_id}")
                });
            },
            #[cfg(debug_assertions)]
            QueryCmd::CurrentName {
                dir: _,
                dirty_ok: _,
            }
            | QueryCmd::ParentId { dir: _ }
            | QueryCmd::ParentName { dir: _, max: _ }
            | QueryCmd::ChildIds { dir: _, max: _ }
            | QueryCmd::ChildId { dir: _ }
            | QueryCmd::CurrentFiles {
                dir: _,
                dirty_ok: _,
            } => todo!(),
            QueryCmd::DirtyFiles { dir: _, clean_ok } => {
                let files = self
                    .plexer
                    .dirty_files(clean_ok)
                    .map_err(CliError::Plexing)?;
                for file in files {
                    writeln!(self.stdout, "{}", file.display()).unwrap_or_else(|_| {
                        panic!("failed stdout write of: {}", file.display())
                    });
                }
            }
            QueryCmd::TrackedFiles { dir: _ } => {
                let files = self
                    .plexer
                    .tracked_files()
                    .map_err(CliError::Plexing)?;
                for file in files {
                    writeln!(self.stdout, "{}", file.display()).unwrap_or_else(|_| {
                        panic!("failed stdout write of: {}", file.display())
                    });
                }
            }
        }
        Ok(0)
    }
}

/// Core logic the CLI binary runs, but with injectable deps; designed fr `main()`'s use-case.
///
/// NOTE: this is separate from main purely so we can e2e (ie: so we can dependency-inject
/// stdio/stderr, etc. into `PlexerQuery`). For more on e2e testing a rust CLI, see:
/// - <https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates>
/// - <https://rust-cli.github.io/book/tutorial/testing.html#testing-cli-applications-by-running-them>
///
/// # Panics
/// Should only panic if stderr or stdout writes fail.
pub fn main_vcsquery(
    args: &MainArgs,
    stdout: &mut dyn io::Write,
    stderr: &mut dyn io::Write,
) -> u8 {
    let plexerq = match PlexerQuery::new(args, stdout) {
        Ok(pq) => pq,
        Err(e) => {
            writeln!(stderr, "{e}").unwrap_or_else(|_| panic!("failed stderr write of: {e}"));
            return 1;
        }
    };
    if let Some(mut pq) = plexerq {
        return match pq.handle_query() {
            Ok(ret) => ret,
            Err(e) => {
                writeln!(stderr, "{e}").unwrap_or_else(|_| panic!("failed stderr write of: {e}"));
                1
            }
        };
    }

    let mut has_fail = false;
    for report in plexer::check_health() {
        let message = match &report.health {
            Ok(h) => h.stdout.clone(),
            Err(e) => e.to_string(),
        };
        if report.health.is_err() {
            writeln!(stderr, "FAIL: check for {:?}:\n{}", report.brand, message)
                .unwrap_or_else(|e| panic!("failed stderr write: {e}"));
            has_fail = true;
        } else {
            writeln!(stdout, "PASS: check for {:?}:\n{}", report.brand, message)
                .unwrap_or_else(|e| panic!("failed stderr write: {e}"));
        }
    }
    u8::from(has_fail)
}

// NOTE: lack of unit tests here, is purely because of the coverage via e2e tests ./tests/
// sub-codebase of this binary target. That doesn't mean unit tests won't be appropriate in this
// file in the future.
