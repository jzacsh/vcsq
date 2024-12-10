/// The particular brands of VCS this library supports.
pub enum VcsBrand {
    Git,
    Mercurial,
    Jujutsu,
}

// TODO needed? helpful?
pub type DirPath = str;

/// Operations any VCS should be able to answer about a repo.
pub trait Repo {
    /// Whether dir is a VCS of this particular branch, or an error if anything went wrong
    /// trying to find out.
    // TODO is returning boolean right here? how can we handle the case that JJ repo is a JJ
    // rpeo, or maybe a JJ-colocated-git repo, or JJ-colocated-p4 repo, or JJ-wrapping-git
    // repo? Just true for all of those? Or some generic type we can define that would let JJ
    // pack the answer here?
    fn is_vcs(dir: &DirPath) -> Result<bool, &str>;

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
