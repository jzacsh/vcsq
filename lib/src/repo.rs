// TODO needed? helpful?
pub type DirPath = std::path::PathBuf;

// TODO(rust) I want is_vcs() to be a function of trait 'Repo' below, but it was causing some
// really strange-loops with the compiler making me deal with https://doc.rust-lang.org/reference/items/traits.html#object-safety
/// Whether dir is a VCS of this particular branch, or an error if anything went wrong
/// trying to find out.
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
    // given dir `foo/` a where is this repo's root?
    //
    // is this repo dirty?
    //
    // what unique ID (commit/rev) can you give me to refer to the repo's current
    // state?
    //
    // what unique bookmark (branch, tag) can you give me to refer to the repo's current?
    //
    // dirty filepaths
    //
    // HEAD's touched files
    // - "touched" means "since last commit"
    //
    // union of the last two
    //
    // HEAD's touched as opposed to "last bookmark"
}
