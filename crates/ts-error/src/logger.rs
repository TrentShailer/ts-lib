//! Extension traits to log errors.

use core::{fmt, panic::Location};

/// Trait to log an error.
#[cfg(feature = "log")]
pub trait LogError {
    /// Log the error.
    #[track_caller]
    fn log_err(self) -> Self;
}

#[cfg(feature = "std")]
/// Trait to log an error to `stderr`.
pub trait StderrError {
    /// Display the error in `stderr`.
    #[track_caller]
    fn stderr_err(self) -> Self;
}

#[cfg(feature = "log")]
impl<T, E: fmt::Display> LogError for Result<T, E> {
    #[track_caller]
    fn log_err(self) -> Self {
        if let Err(error) = self.as_ref() {
            let location = Location::caller();
            log::error!("[{location}] {error}");
        }
        self
    }
}

#[cfg(feature = "std")]
impl<T, E: fmt::Display> StderrError for Result<T, E> {
    #[track_caller]
    fn stderr_err(self) -> Self {
        if let Err(error) = self.as_ref() {
            let location = Location::caller();
            std::eprintln!("{}", ts_ansi::format_error!("[{location}] {error}"));
        }
        self
    }
}

#[cfg(feature = "log")]
impl<T> LogError for Option<T> {
    #[track_caller]
    fn log_err(self) -> Self {
        if self.is_none() {
            let location = Location::caller();
            log::error!("[{location}] value was None");
        }
        self
    }
}

#[cfg(feature = "std")]
impl<T> StderrError for Option<T> {
    #[track_caller]
    fn stderr_err(self) -> Self {
        if self.is_none() {
            let location = Location::caller();
            std::eprintln!("{}", ts_ansi::format_error!("[{location}] value was None"));
        }
        self
    }
}
