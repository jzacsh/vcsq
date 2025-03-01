//! When interacting with text-only CLI APIs, this makes [`std::process::Output`] simpler to
//! interact with, as it encodes your assumption that said APIs aren't going to output non-UTF-8
//! content (eg: like a binary stream).
use std::process::ExitStatus;
use std::process::Output;
use std::string::FromUtf8Error;

/// Provides a lossy wrapper for `std::process:Output` useful for CLIs you're only ever expecting
/// UTF-8 text output from.
///
/// Only useful if the comamnd's UTF-8 failure modes are not critical to your application,
/// otherwise use `Utf8CmdOutput`.
#[derive(Debug)]
pub struct Utf8CmdOutputLossy {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for Utf8CmdOutputLossy {
    fn from(output: Output) -> Self {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Self {
            status,
            stdout,
            stderr,
        }
    }
}

impl Utf8CmdOutputLossy {
    pub fn stdout_strings(&self) -> Vec<String> {
        tty_strings(&self.stdout)
    }

    pub fn stderr_strings(&self) -> Vec<String> {
        tty_strings(&self.stderr)
    }
}

/// Provides a wrapper for `std::process:Output` useful for CLIs you're only ever expecting UTF-8
/// text output from.
///
/// Only useful if you want to be defensive (say `expect()` on any conversion errors), otherwise
/// use `Utf8CmdOutputLossy`.
#[derive(Debug)]
pub struct Utf8CmdOutput {
    pub status: ExitStatus,
    pub stdout: Result<String, FromUtf8Error>,
    pub stdout_lossy: String,
    pub stderr: Result<String, FromUtf8Error>,
    pub stderr_lossy: String,
}

impl From<Output> for Utf8CmdOutput {
    fn from(output: Output) -> Self {
        let lossy = Utf8CmdOutputLossy::from(output.clone());
        Self {
            status: lossy.status,
            stderr: String::from_utf8(output.stderr),
            stderr_lossy: lossy.stderr.clone(),
            stdout: String::from_utf8(output.stdout),
            stdout_lossy: lossy.stdout.clone(),
        }
    }
}

impl Utf8CmdOutput {
    pub fn stdout_strings(&self) -> Result<Vec<String>, FromUtf8Error> {
        let stdout = <Result<String, FromUtf8Error> as Clone>::clone(&self.stdout)?;
        Ok(tty_strings(&stdout))
    }

    pub fn stderr_strings(&self) -> Result<Vec<String>, FromUtf8Error> {
        let stderr = <Result<String, FromUtf8Error> as Clone>::clone(&self.stderr)?;
        Ok(tty_strings(&stderr))
    }
}

fn tty_strings(tty_out: &str) -> Vec<String> {
    tty_out
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
}
