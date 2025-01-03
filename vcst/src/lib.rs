use clap::{Parser, Subcommand};
use libvcst::plexer;
use libvcst::repo::{DirPath, Driver, DriverError};
use std::io;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(
    name = "vcst",
    version,
    about = "vcs queries in rust",
    long_about = "vcst is a rust CLI providing Version Control System (VCS) inspection, without you
needing to know each VCS's proprietary incantations."
)]
pub struct VcstArgs {
    /// Directory for which you'd like to ask VCS questions.
    #[arg(short, long)]
    pub dir: Option<DirPath>,

    #[command(subcommand)]
    pub query: Option<VcstQuery>,
}

#[derive(Error, Debug)]
enum VcstError {
    #[error("usage error: {0}")]
    Usage(String),

    #[error("vcs error: {0}")]
    Plexing(#[from] DriverError),

    #[error("{0}")]
    Unknown(String),
}

impl VcstArgs {
    /// Alternative to clap's parse, just so we can handle defaults
    ///
    // TODO: (feature,clap) fix this clunkiness: somehow allow lone positional arg of a
    // directory for the case that no subcommand is passed. IDK how to do that in clap.
    pub(self) fn reduce(&self) -> Result<VcstQuery, VcstError> {
        if let Some(q) = &self.query {
            Ok(q.clone())
        } else {
            let dir = self
                .dir
                .clone()
                .ok_or(VcstError::Usage(
                    "require either subcmd with a query or a direct --dir".into(),
                ))?
                .clone();
            Ok(VcstQuery::Brand { dir })
        }
    }
}

// TODO: (clap, rust) figure out how to shorten the names of these subcommands so they rust-lang
// naming doesn't turn into annoyingly-long (and hyphenated) names.
//
// TODO: (feature) impl a subcommand that lets you know which $PATH dependencies are found.
#[derive(Debug, Subcommand, Clone)]
pub enum VcstQuery {
    /// Prints the brand of the VCS repo, or exits non-zero if it's not a known VCS repo.
    #[command(arg_required_else_help = true)]
    Brand { dir: DirPath },

    /// Prints the root dir of the repo
    #[command(arg_required_else_help = true)]
    Root { dir: DirPath },

    /// Whether VCS repo is in a clean state, or has uncommitted work.
    #[command(arg_required_else_help = true)]
    IsClean {
        // TODO: (feature) implement subcommand here, eg: enum {diffstat, diff, files}
        dir: DirPath,
    },

    /// Print the VCS repo's current revision ID (eg: rev in Mercurial, ref in git, etc).
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    CurrentId {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        dirty_ok: bool,
    },

    /// Print the VCS repo's current human-readable revision (eg: branch or tag in git, bookmark in
    /// jj)
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    CurrentName {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        dirty_ok: bool,
    },

    /// Print the VCS repo's parent revision ID to the current point in history (eg: rev in
    /// Mercurial, ref in git, etc).
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ParentId { dir: DirPath },

    /// Print the VCS repo's parent revision's human-readable revision name for the first parent it
    /// finds with one, or until it has stepped --max steps. Non-zero exit with no stderr output
    /// indicates one wasn't found.
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    ParentName {
        dir: DirPath,

        /// Max number of parents back to walk when seeking a parent with a hand-written ref name.
        // TODO: (rust) there's a type-way to express positive natural numbers, yeah?
        max: u64,
    },

    /// Lists filepaths touched that are the cause of the repo being dirty, or lists no output if
    /// the repo isn't dirty (thus can be used as a 1:1 proxy for `IsClean`'s behavior).
    #[command(arg_required_else_help = true)]
    DirtyFiles {
        dir: DirPath,
        #[arg(long, default_value_t = false)]
        clean_ok: bool,
        // TODO: (feature) add flag like "--exists" to only show files that are currently present
        // (eg: so this can be piped right to an editor's args).
    },

    /// Prints what files were touched by the `CurrentId`
    #[command(arg_required_else_help = true)]
    #[cfg(debug_assertions)]
    CurrentFiles {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event `IsClean` is false.
        dirty_ok: bool,
        // TODO: (feature) allow an optional Id or Name  (ref or bookmark) of which to compare
        // (instead of just the default which is "parent commit").

        // TODO: (feature) implement subcommand here, eg: enum {diffstat, diff, files} (unified
        // with IsClean)
    },

    /// Prints any system/$PATH info that mgiht be useful for debugging issues this binary might
    /// have onyour system.
    CheckHealth,
}

impl VcstQuery {
    fn dir(&self) -> Option<DirPath> {
        self.dir_path().cloned()
    }

    // TODO: (rust) way to ask clap to make a global positional arg for all these subcommands, so
    // we can rely on its presence?
    fn dir_path(&self) -> Option<&DirPath> {
        match self {
            VcstQuery::Brand { dir }
            | VcstQuery::Root { dir }
            | VcstQuery::IsClean { dir }
            | VcstQuery::DirtyFiles { dir, clean_ok: _ } => Some(dir),
            VcstQuery::CheckHealth => None,
            #[cfg(debug_assertions)]
            VcstQuery::CurrentId { dir, dirty_ok: _ }
            | VcstQuery::CurrentName { dir, dirty_ok: _ }
            | VcstQuery::ParentId { dir }
            | VcstQuery::ParentName { dir, max: _ }
            | VcstQuery::CurrentFiles { dir, dirty_ok: _ } => Some(dir),
        }
    }
}

struct PlexerQuery<'a> {
    plexer: plexer::Repo,
    cli: VcstQuery,
    stdout: &'a mut dyn io::Write,
}

impl<'a> PlexerQuery<'a> {
    fn new(
        args: &'a VcstArgs,
        stdout: &'a mut dyn io::Write,
    ) -> Result<Option<PlexerQuery<'a>>, VcstError> {
        let query = args.reduce()?;
        let Some(dir) = query.dir() else {
            return Ok(None);
        };
        if !dir.is_dir() {
            return Err(VcstError::Usage(
                "dir must be a readable directory".to_string(),
            ));
        }
        let plexer = plexer::Repo::new(&dir)?;
        Ok(Some(PlexerQuery {
            plexer,
            cli: query,
            stdout,
        }))
    }

    pub fn handle_query(&mut self) -> Result<u8, VcstError> {
        match self.cli {
            VcstQuery::Brand { dir: _ } => {
                writeln!(self.stdout, "{:?}", self.plexer.brand)
                    .unwrap_or_else(|_| panic!("failed stdout write of: {:?}", self.plexer.brand));
            }
            VcstQuery::Root { dir: _ } => {
                let root_path = self.plexer.root()?;
                let dir_path = root_path.as_path().to_str().ok_or_else(|| {
                    VcstError::Unknown(format!("vcs generated invalid unicode: {root_path:?}"))
                })?;
                writeln!(self.stdout, "{dir_path}")
                    .unwrap_or_else(|_| panic!("failed stdout write of: {dir_path}"));
            }
            VcstQuery::IsClean { dir: _ } => {
                let is_clean = self.plexer.is_clean().map_err(VcstError::Plexing)?;
                return Ok(u8::from(!is_clean));
            }
            VcstQuery::CheckHealth => panic!("bug: PlexerQuery() should not be constructed for the generalized CheckHealth query"),
            #[cfg(debug_assertions)]
            VcstQuery::CurrentId {
                dir: _,
                dirty_ok: _,
            }
            | VcstQuery::CurrentName {
                dir: _,
                dirty_ok: _,
            }
            | VcstQuery::ParentId { dir: _ }
            | VcstQuery::ParentName { dir: _, max: _ }
            | VcstQuery::CurrentFiles {
                dir: _,
                dirty_ok: _,
            } => todo!(),
            VcstQuery::DirtyFiles { dir: _, clean_ok } => {
                let dirty_files = self
                    .plexer
                    .dirty_files(clean_ok)
                    .map_err(VcstError::Plexing)?;
                for dirty_file in dirty_files {
                    writeln!(self.stdout, "{}", dirty_file.display()).unwrap_or_else(|_| {
                        panic!("failed stdout write of: {}", dirty_file.display())
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
pub fn vcst_query(args: &VcstArgs, stdout: &mut dyn io::Write, stderr: &mut dyn io::Write) -> u8 {
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
