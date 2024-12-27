use clap::Parser;
use libvcst::plexer::RepoPlexer;
use libvcst::repo::DirPath;
use std::path::PathBuf;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Version Control System (VCS) inspection tool.
struct VcstArgs {
    /// Directory for which you'd like to ask VCS questions.
    // TODO: can switch to std::path::Path instead?
    #[arg(short, long)]
    dir: String,

    // TODO next: implement every question in my readme as a flag here, nad match each one to a
    // todo!() macros call in main(). This way we have a nice panic-path to implementing all of
    // Repo trait.
}

// TODO(rust) error infra from the start?
fn from_cli() -> Result<Option<RepoPlexer>, &'static str> {
    let dir: String = VcstArgs::parse().dir;
    let dir: DirPath = PathBuf::from(dir);
    Ok(RepoPlexer::is_vcs(dir)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    match from_cli().expect("could not even construct a plexer") {
        Some(plexer) => {
            println!("appears you have a repo: {:?}", plexer);
        },
        None => {
            eprintln!("dir appears not to be a VCS repo");
        }
    }
    Ok(())
    // TODO setup cargo CLI crate (clippy?)!
}
