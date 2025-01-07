use clap::Parser;
use std::io;
use std::process::exit;
use vcsq::{vcst_query, VcstArgs};

fn main() {
    let exit_code = vcst_query(&VcstArgs::parse(), &mut io::stdout(), &mut io::stderr());
    exit(exit_code.into());
}
