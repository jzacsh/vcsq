use crate::repo::{
    Driver, DriverError, HistoryRefId, HistoryRefName, QueryDir, Validator, VcsAvailable,
    ERROR_REPO_NOT_CLEAN, ERROR_REPO_NOT_DIRTY,
};
use std::path::PathBuf;
use std::process::{Command, Stdio};

static VCS_BIN_NAME: &str = "jj";

#[derive(Debug)]
pub struct Repo {
    dir: QueryDir,
}

#[derive(Debug)]
pub struct Loader
where
    Self: Sized;

impl Validator for Loader {
    fn new_driver(&self, dir: QueryDir) -> Result<Option<Box<dyn Driver>>, DriverError> {
        let repo = Repo { dir };

        let is_ok = DriverError::unwrap_cmd_lossy(
            "jj cli".to_string(),
            repo.jj_root()
                // TODO: (feature) check 'output.stdout' is a non-empty substr of 'dir'
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output(),
        )?
        .status
        .success();
        if is_ok {
            let repo: Box<dyn Driver> = Box::from(repo);
            Ok(Some(repo))
        } else {
            Ok(None)
        }
    }

    fn check_health(&self) -> Result<VcsAvailable, DriverError> {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.arg("--version");
        DriverError::expect_cmd_lossy("jj cli: exec".to_string(), cmd.output())
    }
}

impl Repo {
    fn start_shellout(&self) -> Command {
        let mut cmd = Command::new(VCS_BIN_NAME);
        cmd.current_dir(self.dir.clone());
        cmd
    }

    fn jj_root(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("root");
        cmd
    }

    fn jj_dirty_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("diff").arg("--name-only");
        cmd
    }

    fn jj_tracked_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        // TODO: unclear @- is always the right bet. _Sometimes_ you can put yourself into a weird
        // state where your '@' is actually not just an ephemeral copy, as you've re-attached
        // yourself to it.
        cmd.arg("file").arg("list").arg("-r").arg("@-");
        cmd
    }

    fn jj_current_ref_id(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("log")
            .arg("--color=never")
            .arg("--no-graph")
            .arg("--revisions")
            .arg("@-")
            .arg("--template")
            .arg("commit_id");
        cmd
    }
}

impl Driver for Repo {
    fn root(&self) -> Result<QueryDir, DriverError> {
        let output = DriverError::expect_cmd_lossy("jj cli".to_string(), self.jj_root().output())?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "jj cli", &output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<QueryDir>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.jj_dirty_files().output(),
            min_lines,
            "jj cli: exec",
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines.into_iter().map(PathBuf::from).collect();
        Ok(dirty_files)
    }

    fn tracked_files(&self) -> Result<Vec<QueryDir>, DriverError> {
        let lines = DriverError::expect_cmd_lines(
            self.jj_tracked_files().output(),
            0, /*min_lines*/
            "jj cli: exec",
            None,
        )?;
        let files = lines.into_iter().map(PathBuf::from).collect();
        Ok(files)
    }

    /// Returns the backing store's "commit id" (as opposed to the more ephemeral "change id").
    fn current_ref_id(&self, dirty_ok: bool) -> Result<HistoryRefId, DriverError> {
        if !dirty_ok && !self.is_clean()? {
            return Err(ERROR_REPO_NOT_CLEAN.to_string().into());
        }
        let output =
            DriverError::expect_cmd_lossy("jj cli".to_string(), self.jj_current_ref_id().output())?;
        DriverError::expect_cmd_line("jj cli", &output)
    }

    /// Returns the current bookmark if there one.
    // TODO: (feature) when jj is more stable, do more advanced things: determine if we're
    // git-backed, and then translate our answers into the answe ra user would expect if this
    // iwasn't jj-on-git but just git. For now we just keep it sipmle (and less featureful) then
    // our git counterpart driver.
    fn current_ref_name(&self, _dirty_ok: bool) -> Result<Option<HistoryRefName>, DriverError> {
        todo!(); // TODO: (feature) implement
    }
}
