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
fn fromCli() -> Result<RepoPlexer, &'static str> {
    let dir: String = VcstArgs::parse().dir;
    let dir: DirPath = PathBuf::from(dir);
    let plexer = RepoPlexer::from(dir)?;
    Ok(plexer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let plexer = fromCli()?;
    println!("zomg write some calls for $PWD already!\n\t{:?}", plexer);
    println!("\tDO NOT SUBMIT setup cargo CLI crate (clippy?)!");
    println!("\tin the meantime, here's whether you're in a VCS repo:");

    Ok(())
}
