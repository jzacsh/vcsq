use clap::{Parser,Subcommand};
use libvcst::plexer::RepoPlexer;
use libvcst::repo::DirPath;
use std::path::PathBuf;
use std::error::Error;
use std::process::exit;

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
    dir: Option<String>, // TODO: can switch to std::path::Path or DirPath instead?

    #[command(subcommand)]
    query: Option<VcstQuery>,

    // TODO next: implement every question in my readme as a flag here, nad match each one to a
    // todo!() macros call in main(). This way we have a nice panic-path to implementing all of
    // Repo trait.
//  1. HEAD's touched files
//     - "touched" means "since last commit"
//  1. union of the last two
//  1. HEAD's touched as opposed to "last bookmark"

}

impl VcstArgs {
    /// Alternative to clap's parse, just so we can handle defaults
    pub fn init() -> Result<VcstArgs, &'static str> {
        let mut args = VcstArgs::parse();
        // TODO(rust) delete all this clunky method if Clap has a way to express defaults directly
        // with its derive macros. 
        if let Some(ref dir) = args.dir {
            if args.query.is_none() {
                args.query = Some(VcstQuery::Brand{dir: dir.clone()});
            }
            Ok(args)
        } else {
            // TODO(feature) impl a subcommand that lets you know which $PATH dependencies are
            // found.


            // TODO(rust) is this really the clap way? some "usage error" type that lets clap
            // do other nice things?
            Err("usage: dir is required")
        }
    }
}

// TODO(clap, rust) figure out how to shorten the names of these subcommands so they rust-lang
// naming doesn't turn into annoyingly-long (and hyphenated) names.
#[derive(Debug, Subcommand, Clone)]
enum VcstQuery {
    /// Prints the brand of the VCS repo, or exits non-zero if it's not a known VCS repo.
    #[command(arg_required_else_help = true)]
    Brand { dir: String }, // TODO: can switch to std::path::Path or DirPath instead?

    /// Prints the root dir of the repo
    #[command(arg_required_else_help = true)]
    Root { dir: String }, // TODO: can switch to std::path::Path or DirPath instead?

    /// Whether VCS repo is in a clean state, or has uncommitted work.
    #[command(arg_required_else_help = true)]
    IsClean {
        // TODO(feature) implement subcommand here, eg: enum {diffstat, diff, files}
        dir: String // TODO: can switch to std::path::Path or DirPath instead?
    },

    /// Print the VCS repo's current revision ID (eg: rev in Mercurial, ref in git, etc).
    #[command(arg_required_else_help = true)]
    CurrentId {
        dir: String, // TODO: can switch to std::path::Path or DirPath instead?

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirtyOk: bool,
    },

    /// Print the VCS repo's current human-readable revision (eg: branch or tag in git, bookmark in
    /// jj)
    #[command(arg_required_else_help = true)]
    CurrentName {
        dir: String, // TODO: can switch to std::path::Path or DirPath instead?

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirtyOk: bool,
    },

    /// Lists filepaths touched that are the cause of the repo being dirty, or lists no output is
    /// the repo isn't dirty (thus can be used as a 1:1 proxy for IsClean's behavior).
    #[command(arg_required_else_help = true)]
    DirtyFiles {
        dir: String, // TODO: can switch to std::path::Path or DirPath instead?
    },

    /// Prints what files were touched by the CurrentId
    CurrentFiles {
        dir: String, // TODO: can switch to std::path::Path or DirPath instead?

        /// Whether to be silent about any answers being flawed, in the event IsClean is false.
        dirtyOk: bool,

        // TODO(feature) allow an optional Id or Name  (ref or bookmark) of which to compare
        // (instead of just the default which is "parent commit").

        // TODO(feature) implement subcommand here, eg: enum {diffstat, diff, files} (unified with
        // IsClean)
    }
}


struct PlexerQuery {
    plexer: Option<RepoPlexer>,
    cli: VcstArgs,
}

// TODO(rust) error infra from the start?
fn from_cli() -> Result<PlexerQuery, String> {
    let vcst_args = VcstArgs::init()?;
    let dir: String = vcst_args.dir.clone().unwrap();
    let dir: DirPath = PathBuf::from(dir);
    let plexer = RepoPlexer::new(dir)?;
    Ok(PlexerQuery{
      plexer,
      cli: vcst_args,
    })
}

// TODO(rust/clap): what happens with all errors we unwrap? should we just do
// https://doc.rust-lang.org/1.61.0/std/process/struct.ExitCode.html#examples instead?
// TODO(rust) setup clippy somehow?
fn main() -> Result<(), Box<dyn Error>> {
    let vcst_query = match from_cli() {
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
        Ok(vcst) => vcst,
    };
    let plexer = match vcst_query.plexer {
        None => {
            eprintln!("dir appears not to be a VCS repo");
            exit(1);
        },
        Some(plexer) => plexer,
    };
    match vcst_query.cli.query.expect("bug: init() should have guaranteed a query") {
        VcstQuery::Brand{dir} => {
            println!("{:?}", plexer.brand);
        },
        VcstQuery::Root{dir} => todo!(),
        VcstQuery::IsClean{dir} => todo!(),
        VcstQuery::CurrentId{dir, dirtyOk} => todo!(),
        VcstQuery::CurrentName{dir, dirtyOk} => todo!(),
        VcstQuery::DirtyFiles{dir} => todo!(),
        VcstQuery::CurrentFiles{dir, dirtyOk} => todo!(),
    }
    Ok(())
}
