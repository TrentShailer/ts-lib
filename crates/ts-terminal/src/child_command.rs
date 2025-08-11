//! Easy wrapper to process some data using a child process.

use std::{
    ffi::OsStr,
    io::{self, Write},
    process::{Command, ExitStatus, Stdio},
    thread,
};

/// Error variants for using a child command.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ChildCommandError {
    #[non_exhaustive]
    SpawnChild { source: io::Error },

    #[non_exhaustive]
    WriteToStdin { source: io::Error },

    #[non_exhaustive]
    ReadOutput { source: io::Error },

    #[non_exhaustive]
    UnsuccessfulStatus { status: ExitStatus, stderr: String },
}
impl core::fmt::Display for ChildCommandError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::SpawnChild { .. } => write!(f, "could not spawn child process"),
            Self::WriteToStdin { .. } => write!(f, "writing to child's stdin failed"),
            Self::ReadOutput { .. } => write!(f, "reading child's output failed"),
            Self::UnsuccessfulStatus { status, stderr, .. } => write!(
                f,
                "child process reported exit code {status:?}, with stderr: {stderr}"
            ),
        }
    }
}
impl core::error::Error for ChildCommandError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ReadOutput { source, .. }
            | Self::WriteToStdin { source, .. }
            | Self::SpawnChild { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Write `data` to a child process' `stdin`, and return the process' `stdout`.
///
/// ## Panics
/// * If handle to child's `stdin` could not be taken.
/// * If the writer thread panics.
pub fn process_using_child<C: AsRef<OsStr>, I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
    command: C,
    args: I,
    data: &[u8],
) -> Result<Vec<u8>, ChildCommandError> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|source| ChildCommandError::SpawnChild { source })?;

    let mut stdin = child.stdin.take().expect("stdin handle to be present");
    let output = thread::scope(|s| {
        let writer = s.spawn(move || stdin.write_all(data));

        let output = child
            .wait_with_output()
            .map_err(|source| ChildCommandError::ReadOutput { source });

        let write_result = writer
            .join()
            .expect("writer thread to not panic")
            .map_err(|source| ChildCommandError::WriteToStdin { source });

        if let Some(error) = write_result.err() {
            return Err(error);
        }

        output
    })?;

    if !output.status.success() {
        return Err(ChildCommandError::UnsuccessfulStatus {
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(output.stdout)
}
