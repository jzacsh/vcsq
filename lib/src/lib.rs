// TODO finish converting readme bulletted list to proper API surfaces/docs below (then update the
// readme to point here as the canonical reference).
pub mod repo;

/// is dir `foo/` a VCS repo?
/// if so, of which type?
pub fn vcs_type(dir: &repo::DirPath) -> Result<repo::VcsBrand, &str> {
    todo!();
}
