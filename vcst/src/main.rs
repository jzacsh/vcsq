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
}

// TODO(rust) error infra from the start?
fn fromCli() -> Result<Option<RepoPlexer>, &'static str> {
    let dir: String = VcstArgs::parse().dir;
    let dir: DirPath = PathBuf::from(dir);
    RepoPlexer::is_vcs(dir)?
}

fn main() -> Result<(), Box<dyn Error>> {
    let plexer = fromCli().expect("could not even construct a plexer");
    println!("zomg write some calls for $PWD already!\n\t{:?}", plexer);
    println!("\tDO NOT SUBMIT setup cargo CLI crate (clippy?)!");
    println!("\tin the meantime, here's whether you're in a VCS repo:");
    println!("plexer#is_vcs: {:?}", plexer.is_vcs().expect("able to ask if dir '{:?}' is a vcs"))

    Ok(())
}
