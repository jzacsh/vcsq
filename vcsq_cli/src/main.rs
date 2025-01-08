//! vcsq CLI, exposing most of the abilities of [`vcsq_lib`] library, for scripting.
use clap::Parser;
use std::io;
use std::process::exit;
use vcsq_cli::{vcst_query, VcstArgs};

fn main() {
    let exit_code = vcst_query(&VcstArgs::parse(), &mut io::stdout(), &mut io::stderr());
    exit(exit_code.into());
}
