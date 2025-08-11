//! # `ts-error`
//!
//! Traits for convenient error reporting, and error report/stack creation

#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod diagnostic;
mod logger;
mod program_exit;
mod report;

use alloc::string::{String, ToString};

#[cfg(feature = "log")]
pub use logger::LogError;
pub use program_exit::{ProgramReport, ReportProgramExit};
pub use report::{IntoReport, Report};

#[cfg(feature = "std")]
pub use logger::StderrError;

/// Normalize an error message.
/// * Starts with lowercase character unless followed by an uppercase character.
/// * Does not end with any punctuation.
pub fn normalize_message<S: ToString>(message: S) -> String {
    let message = message.to_string();
    let message = message.trim();
    let mut output = String::with_capacity(message.len());

    let mut chars = message.chars();
    let first_char = chars.next();
    let second_char = chars.next();

    // Handle acronyms
    if let Some(first_char) = first_char
        && let Some(second_char) = second_char
        && first_char.is_uppercase()
        && !second_char.is_uppercase()
    {
        output.push(first_char.to_ascii_lowercase());
        output.push(second_char);
    } else {
        if let Some(first_char) = first_char {
            output.push(first_char);
        }
        if let Some(second_char) = second_char {
            output.push(second_char);
        }
    }

    let mut chars = chars.rev().peekable();
    // Skip trailing punctuation
    while chars.next_if(char::is_ascii_punctuation).is_some() {}

    output.push_str(&chars.rev().collect::<String>());

    output.trim().to_string()
}

#[cfg(test)]
mod test {
    use crate::normalize_message;

    #[test]
    fn does_not_normalize_acronyms() {
        let message = "JSON";
        assert_eq!(message, normalize_message(message));

        let message = "  JSON  ";
        assert_eq!("JSON", normalize_message(message));
    }

    #[test]
    fn normalizes_sentences() {
        let message = "Whether";
        assert_eq!("whether", normalize_message(message));

        let message = "  Whether  ";
        assert_eq!("whether", normalize_message(message));
    }

    #[test]
    fn removes_punctuation() {
        let message = "message.,;/";
        assert_eq!("message", normalize_message(message));

        let message = "  message .,;/  ";
        assert_eq!("message", normalize_message(message));
    }
}
