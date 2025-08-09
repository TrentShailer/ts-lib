//! # `ts-io`
//!
//! Helpers for input/output related work.

extern crate alloc;

mod cursor;
mod read_file;

pub use cursor::{Cursor, OutOfBounds};
pub use read_file::{ReadFileError, read_file, read_file_to_string};
