use std::process::ExitStatus;
use std::process::Output;
use std::string::FromUtf8Error;

/// Provides a lossy wrapper for `std::process:Output` useful for CLIs you're only ever expecting
/// utf8-text output from.
///
/// Only useful if the comamnd's utf8-failure modes are not critical to your applciation, otherwise
/// use `Utf8CmdOutput`.
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
            stderr,
            stdout,
        }
    }
}

/// Provides a wrapper for `std::process:Output` useful for CLIs you're only ever expecting
/// utf8-text output from.
///
/// Only useful if you want to be defensive (say `expect()` on any conversion errors), otherwise
/// use `Utf8CmdOutputLossy`.
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
