//! # `ts-ansi`
//!
//! Constant ANSI codes for easy styling and terminal printing helpers.

mod action;
pub mod style;

pub use action::{Action, ActionResult};
pub use strip_ansi_escapes;
