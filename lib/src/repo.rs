// TODO needed? helpful?
pub type DirPath = std::path::PathBuf;

// TODO(rust) common error types here that we want all Repo's to return in their constructions

// TODO is returning boolean right here? how can we handle the case that JJ repo is a JJ
// rpeo, or maybe a JJ-colocated-git repo, or JJ-colocated-p4 repo, or JJ-wrapping-git
// repo? Just true for all of those? Or some generic type we can define that would let JJ
// pack the answer here?
// TDOO design error return type that can distinguish between OS/access errors, and simply :
// no, this isn't a Self repo.
//fn is_vcs(dir: DirPath) -> Result<Option<Self>, &'static str>;


/// Operations any VCS should be able to answer about a repo.
// TODO finish convert from readme list to proper API surfaces/docs below (then update the
// readme to point here as the canonical reference).
pub trait Repo
    where Self: std::fmt::Debug,
{

}
