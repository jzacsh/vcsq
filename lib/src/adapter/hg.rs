use crate::repo::{
    DirPath, Driver, DriverError, HistoryRefId, Validator, VcsAvailable, ERROR_REPO_NOT_DIRTY,
};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Repo {
    dir: DirPath,
}

static VCS_BIN_NAME: &str = "hg";

const HG_LOGID_DIRTY_SUFFIX: &str = "+";

fn start_vcs_shellout() -> Command {
    let mut cmd = Command::new(VCS_BIN_NAME);
    cmd.env("HGPLAIN", "1");
    cmd
}

#[derive(Debug)]
pub struct Loader
where
    Self: Sized;

impl Validator for Loader {
    fn new_driver(&self, dir: DirPath) -> Result<Option<Box<dyn Driver>>, DriverError> {
        let repo = Repo { dir };

        let is_ok = DriverError::unwrap_cmd_lossy(
            "hg cli".to_string(),
            repo.hg_root()
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
        let mut cmd = start_vcs_shellout();
        cmd.arg("--version");
        DriverError::expect_cmd_lossy("hg cli: exec".to_string(), cmd.output())
    }
}

impl Repo {
    fn start_shellout(&self) -> Command {
        let mut cmd = start_vcs_shellout();
        cmd.current_dir(self.dir.clone());
        cmd
    }

    fn hg_root(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("root");
        cmd
    }

    fn hg_dirty_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("status")
            .arg("--modified")
            .arg("--added")
            .arg("--removed")
            .arg("--deleted")
            .arg("--unknown");
        cmd
    }

    fn hg_tracked_files(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("status").arg("--all");
        cmd
    }

    fn hg_current_id(&self) -> Command {
        let mut cmd = self.start_shellout();
        cmd.arg("--debug").arg("id").arg("-i");
        cmd
    }
}

impl Driver for Repo {
    fn root(&self) -> Result<DirPath, DriverError> {
        let output =
            DriverError::expect_cmd_lossy("hg cli: exec".to_string(), self.hg_root().output())?;
        Ok(PathBuf::from(DriverError::expect_cmd_line(
            "hg cli", &output,
        )?))
    }

    fn dirty_files(&self, clean_ok: bool) -> Result<Vec<DirPath>, DriverError> {
        let min_lines = u8::from(!clean_ok);
        let lines = DriverError::expect_cmd_lines(
            self.hg_dirty_files().output(),
            min_lines,
            "hg cli: exec",
            Some(ERROR_REPO_NOT_DIRTY.to_string()),
        )?;
        let dirty_files = lines
            .into_iter()
            .map(|ln| {
                // first 2 chars are modification-indicators like "?? " to indicate the file is
                // untracked.
                ln.chars().skip(2).collect::<String>()
            })
            .map(PathBuf::from)
            .collect();
        Ok(dirty_files)
    }

    fn tracked_files(&self) -> Result<Vec<DirPath>, DriverError> {
        let lines = DriverError::expect_cmd_lines(
            self.hg_tracked_files().output(),
            0, /*min_lines*/
            "hg cli: exec",
            None,
        )?;
        // first 2 chars ar the files status codes, of which the one we car eabout from the
        // docs is: "C = clean". We look for those lines, then strip that status information.
        let files = lines
            .into_iter()
            .filter(|ln| ln.starts_with("C "))
            .map(|ln| ln.chars().skip(2).collect::<String>())
            .map(PathBuf::from)
            .collect();
        Ok(files)
    }

    fn current_ref_id(&self, dirty_ok: bool) -> Result<HistoryRefId, DriverError> {
        if !dirty_ok {
            todo!(); // TODO: implement dirty_ok check
        }
        let output = DriverError::expect_cmd_lossy(
            "hg cli :exec".to_string(),
            self.hg_current_id().output(),
        )?;
        let current_id = DriverError::expect_cmd_line("hg cli: exec", &output)?;
        if !current_id.ends_with(HG_LOGID_DIRTY_SUFFIX) {
            return Ok(current_id);
        }
        Ok(current_id
            .strip_suffix(HG_LOGID_DIRTY_SUFFIX)
            .ok_or_else(|| {
                DriverError::Unknown(format!("hg bug? got just a lone '{HG_LOGID_DIRTY_SUFFIX}'"))
            })?
            .to_string())
    }
}
