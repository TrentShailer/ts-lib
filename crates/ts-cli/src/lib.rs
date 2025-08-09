//! # `ts-ansi`
//!
//! Helpers for creating my CLIs

extern crate alloc;

mod action;
mod child_command;

pub use action::{Action, ActionResult};
pub use child_command::{ChildCommandError, process_using_child};
