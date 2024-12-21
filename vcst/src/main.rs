use clap::Parser;

/// Version Control System (VCS) inspection tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct VcstArgs {
    /// Directory for which you'd like to ask VCS questions.
    // TODO: can swithc to std::path::Path instead?
    #[arg(short, long)]
    dir: String,
}

fn main() {
    println!("zomg write some calls for $PWD already!\n\t{:?}", VcstArgs::parse());
    println!("\tDO NOT SUBMIT setup cargo CLI crate (clippy?)!");
    println!("\tin the meantime, here's whether you're in a VCS repo:");
}
