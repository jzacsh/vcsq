use clap::{Parser, Subcommand};
use libvcst::plexer::RepoPlexer;
use libvcst::repo::{DirPath, Repo, RepoLoadError};
use std::path::PathBuf;
use std::process::exit;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(
    name = "vcst",
    version,
    about = "vcs queries in rust",
    long_about = "vcst is a rust CLI providing Version Control System (VCS) inspection, without you
needing to know each VCS's proprietary incantations."
)]
struct VcstArgs {
    /// Directory for which you'd like to ask VCS questions.
    #[arg(short, long)]
    dir: Option<DirPath>,

    #[command(subcommand)]
    query: Option<VcstQuery>,
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
    pub fn reduce(&self) -> Result<VcstQuery, VcstError> {
        match &self.query {
            Some(q) => Ok(q.clone()), // TODO: (rust) can we args.query.unwrap_or_else() but that can accept errors?
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
enum VcstQuery {
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

    /// Lists filepaths touched that are the cause of the repo being dirty, or lists no output is
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

struct PlexerQuery {
    plexer: Option<RepoPlexer>,
    cli: VcstQuery,
}

fn from_cli() -> Result<PlexerQuery, VcstError> {
    let query = VcstArgs::parse().reduce()?;
    let dir: String = query.dir()?;
    let dir: DirPath = PathBuf::from(dir);
    let plexer = RepoPlexer::new(dir)?;
    Ok(PlexerQuery { plexer, cli: query })
}

// TODO: (rust/clap): what happens with all errors we unwrap? should we just do
// https://doc.rust-lang.org/1.61.0/std/process/struct.ExitCode.html#examples instead?
// TODO: (rust) setup clippy somehow?
fn main() -> Result<(), VcstError> {
    let vcst_query = match from_cli() {
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
        Ok(vcst) => vcst,
    };
    let plexer = match vcst_query.plexer {
        None => {
            eprintln!("dir appears not to be a VCS repo");
            exit(1);
        }
        Some(plexer) => plexer,
    };

    match vcst_query.cli {
        VcstQuery::Brand { dir: _ } => {
            println!("{:?}", plexer.brand);
        }
        VcstQuery::Root { dir: _ } => match plexer.root() {
            Ok(root_path) => {
                let dir_path = root_path.as_path().to_str().ok_or_else(|| {
                    VcstError::Unknown(format!("vcs generated invalid unicode: {:?}", root_path))
                })?;
                println!("{}", dir_path);
            }
            Err(e) => {
                eprintln!("root dir: {:?}", e);
                exit(1);
            }
        },
        VcstQuery::IsClean { dir: _ } => todo!(),
        VcstQuery::CurrentId {
            dir: _,
            dirty_ok: _,
        } => todo!(),
        VcstQuery::CurrentName {
            dir: _,
            dirty_ok: _,
        } => todo!(),
        VcstQuery::DirtyFiles { dir: _ } => todo!(),
        VcstQuery::CurrentFiles {
            dir: _,
            dirty_ok: _,
        } => todo!(),
    }
    Ok(())
}
