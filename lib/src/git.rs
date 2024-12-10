use crate::repo;

pub struct RepoGit {}

impl struct repo::Repo for RepoGit {
    fn is_vcs(dir: &DirPath) -> Result<bool, &str> {
        todo!(); // DO NOT SUBMIT: just shell out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
