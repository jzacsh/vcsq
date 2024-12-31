use clap::{Parser, Subcommand};
use libvcst::plexer::RepoPlexer;
use libvcst::repo::{DirPath, Repo, RepoLoadError};
use std::io;
use std::path::PathBuf;
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
    Plexing(#[from] RepoLoadError),

    #[error("{0}")]
    Unknown(String),
}

impl VcstArgs {
    /// Alternative to clap's parse, just so we can handle defaults
    ///
    // TODO: (feature,clap) fix this clunkiness: somehow allow lone positional arg of a
    // directory for the case that no subcommand is passed. IDK how to do that in clap.
    pub(self) fn reduce(&self) -> Result<VcstQuery, VcstError> {
        match &self.query {
            // TODO: (rust) is there an "Option, else default" pattern that lets errors bubble out
            // of the closure? eg: can we self.query.unwrap_or_else() but that can accept errors
            // (as in the error-cases in the None match-arm below)?
            Some(q) => Ok(q.clone()),
            None => {
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
    CurrentId {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirty_ok: bool,
    },

    /// Print the VCS repo's current human-readable revision (eg: branch or tag in git, bookmark in
    /// jj)
    #[command(arg_required_else_help = true)]
    CurrentName {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirty_ok: bool,
    },

    /// Lists filepaths touched that are the cause of the repo being dirty, or lists no output if
    /// the repo isn't dirty (thus can be used as a 1:1 proxy for IsClean's behavior).
    #[command(arg_required_else_help = true)]
    DirtyFiles { dir: DirPath },

    /// Prints what files were touched by the CurrentId
    CurrentFiles {
        dir: DirPath,

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirty_ok: bool,
        // TODO: (feature) allow an optional Id or Name  (ref or bookmark) of which to compare
        // (instead of just the default which is "parent commit").

        // TODO: (feature) implement subcommand here, eg: enum {diffstat, diff, files} (unified
        // with IsClean)
    },
}

impl VcstQuery {
    fn dir(&self) -> Result<String, VcstError> {
        return Ok(self
            .dir_path()
            .to_str()
            .ok_or_else(|| {
                VcstError::Usage(format!(
                    "invalid unicode found in dir: {:?}",
                    self.dir_path()
                ))
            })?
            .to_string());
    }

    // TODO: (rust) way to ask clap to make a global positional arg for all these subcommands, so
    // we can rely on its presence?
    fn dir_path(&self) -> &DirPath {
        match self {
            VcstQuery::Brand { dir } => dir,
            VcstQuery::Root { dir } => dir,
            VcstQuery::IsClean { dir } => dir,
            VcstQuery::CurrentId { dir, dirty_ok: _ } => dir,
            VcstQuery::CurrentName { dir, dirty_ok: _ } => dir,
            VcstQuery::DirtyFiles { dir } => dir,
            VcstQuery::CurrentFiles { dir, dirty_ok: _ } => dir,
        }
    }
}

struct PlexerQuery<'a> {
    plexer: RepoPlexer,
    cli: VcstQuery,
    stdout: &'a mut dyn io::Write,
}

impl PlexerQuery<'_> {
    fn new<'a>(
        args: VcstArgs,
        stdout: &'a mut dyn io::Write,
    ) -> Result<PlexerQuery<'a>, VcstError> {
        let query = args.reduce()?;
        let dir: String = query.dir()?;
        let dir: DirPath = PathBuf::from(dir);
        if !dir.is_dir() {
            return Err(VcstError::Usage(
                "dir must be a readable directory".to_string(),
            ));
        }
        let plexer = RepoPlexer::new(dir)?;
        Ok(PlexerQuery {
            plexer,
            cli: query,
            stdout,
        })
    }

    pub fn handle_query(&mut self) -> Result<(), VcstError> {
        match self.cli {
            VcstQuery::Brand { dir: _ } => {
                write!(self.stdout, "{:?}\n", self.plexer.brand)
                    .expect(format!("failed stdout write of: {:?}", self.plexer.brand).as_str());
            }
            VcstQuery::Root { dir: _ } => {
                match self.plexer.root() {
                    Ok(root_path) => {
                        let dir_path = root_path.as_path().to_str().ok_or_else(|| {
                            return VcstError::Unknown(format!(
                                "vcs generated invalid unicode: {:?}",
                                root_path
                            ));
                        })?;
                        write!(self.stdout, "{}\n", dir_path)
                            .expect(format!("failed stdout write of: {}", dir_path).as_str());
                    }
                    Err(e) => {
                        return Err(VcstError::Unknown(format!("root dir: {:?}", e)));
                    }
                };
            }
            VcstQuery::IsClean { dir: _ } => todo!(),
            VcstQuery::CurrentId {
                dir: _,
                dirty_ok: _,
            } => todo!(),
            VcstQuery::CurrentName {
                dir: _,
                dirty_ok: _,
            } => todo!(),
            VcstQuery::DirtyFiles { dir: _ } => {
                let dirty_files = self
                    .plexer
                    .dirty_files()
                    .map_err(|e| VcstError::Plexing(e))?;
                for dirty_file in dirty_files {
                    write!(self.stdout, "{}\n", dirty_file.display()).expect(
                        format!("failed stdout write of: {}", dirty_file.display()).as_str(),
                    );
                }
            }
            VcstQuery::CurrentFiles {
                dir: _,
                dirty_ok: _,
            } => todo!(),
        }
        Ok(())
    }
}

/// Core logic the CLI binary runs, but with injectable deps.
///
/// NOTE: this is separate from main purely so we can e2e (ie: so we can dependency-inject
/// stdio/stderr, etc. into PlexerQuery). For more on e2e testing a rust CLI, see:
/// - https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates
/// - https://rust-cli.github.io/book/tutorial/testing.html#testing-cli-applications-by-running-them
pub fn vcst_query(args: VcstArgs, stdout: &mut dyn io::Write, stderr: &mut dyn io::Write) -> u8 {
    let mut plexerq = match PlexerQuery::new(args, stdout) {
        Ok(pq) => pq,
        Err(e) => {
            write!(stderr, "{}\n", e).expect(format!("failed stderr write of: {}", e).as_str());
            return 1;
        }
    };
    match plexerq.handle_query() {
        Ok(_) => {}
        Err(e) => {
            write!(stderr, "{}\n", e).expect(format!("failed stderr write of: {}", e).as_str());
            return 1;
        }
    };
    return 0;
}

// NOTE: lack of unit tests here, is purely because of the coverage via e2e tests ./tests/
// sub-codebase of this binary target. That doesn't mean unit tests won't be appropriate in this
// file in the future.
