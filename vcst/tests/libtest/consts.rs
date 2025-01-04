pub static ERROR_NO_KNOWN_VCS: &str =
    "vcs error: if dir is a VCS, it\'s of an unknown brand (tried these 3: Git, Mercurial, Jujutsu)";

pub static ERROR_NOT_VALID_DIR: &str = "usage error: dir must be a readable directory";

pub static ERROR_DIR_MISSING: &str =
    "usage error: require either subcmd with a query or a direct --dir";
