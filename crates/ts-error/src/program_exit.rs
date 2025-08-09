use alloc::boxed::Box;
use core::{error::Error, fmt};

use crate::Report;

/// Type alias for a program that reports it's exit.
pub type ReportProgramExit = Result<(), ProgramReport>;

/// A report for a program exit.
pub struct ProgramReport(Box<dyn Error + 'static>);
impl<E: Error + 'static> From<E> for ProgramReport {
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}
impl fmt::Debug for ProgramReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl fmt::Display for ProgramReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let report = Report::new(self.0.as_ref());

        #[cfg(feature = "std")]
        let current_exe = std::env::current_exe().ok();
        #[cfg(feature = "std")]
        let current_exe_file_name = current_exe.as_ref().and_then(|path| path.file_name());
        #[cfg(feature = "std")]
        let current_exe_name = current_exe_file_name
            .as_ref()
            .and_then(|name| name.to_str());
        #[cfg(feature = "std")]
        let exe = current_exe_name.unwrap_or("the program");
        #[cfg(not(feature = "std"))]
        let exe = "the program";

        writeln!(f, "{exe} exited unsuccessfully")?;
        write!(f, "{report}")
    }
}
