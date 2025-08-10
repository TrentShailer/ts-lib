use alloc::boxed::Box;
use core::{error::Error, fmt};

use ts_ansi::style::{BOLD, DEFAULT, RED, RESET};

/// Trait for converting something into an error report.
pub trait IntoReport<T> {
    /// Convert self into an error report if self is an error.
    fn into_report(self) -> Result<T, Report<'static>>;
}

impl<T, E: Error + 'static> IntoReport<T> for Result<T, E> {
    fn into_report(self) -> Result<T, Report<'static>> {
        self.map_err(|source| Report::new(source))
    }
}

/// An error report, displays the error stack of some error.
pub struct Report<'e> {
    /// The error for this report.
    pub source: Box<dyn Error + 'e>,
}
impl<'e> Report<'e> {
    /// Create a new error report.
    pub fn new<E: Error + 'e>(source: E) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}
impl Error for Report<'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
impl fmt::Debug for Report<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl fmt::Display for Report<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_error = Some(self.source.as_ref());
        let mut count = 1;

        while let Some(error) = current_error {
            writeln!(f, " {BOLD}{RED}{count}{DEFAULT}.{RESET} {error}")?;

            count += 1;
            current_error = error.source();
        }

        Ok(())
    }
}
