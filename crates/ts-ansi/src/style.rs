//! ANSI codes

use core::fmt::{Display, Write};

/// Reset styling
pub const RESET: &str = "\x1b[0m";

/// Following text will be bold
pub const BOLD: &str = "\x1b[1m";
/// Following text will NOT be bold
pub const NO_BOLD: &str = "\x1b[21m";

/// Following text will be dim
pub const DIM: &str = "\x1b[2m";
/// Following text will NOT be dim
pub const NO_DIM: &str = "\x1b[22m";

/// Set colour of text to black
pub const BLACK: &str = "\x1b[90m";
/// Set background of text to black
pub const BG_BLACK: &str = "\x1b[100m";
/// Set colour of text to dim black
pub const DIM_BLACK: &str = "\x1b[30m";
/// Set background of text to dim black
pub const BG_DIM_BLACK: &str = "\x1b[40m";

/// Set colour of text to red
pub const RED: &str = "\x1b[91m";
/// Set background of text to red
pub const BG_RED: &str = "\x1b[101m";
/// Set colour of text to dim red
pub const DIM_RED: &str = "\x1b[31m";
/// Set background of text to dim red
pub const BG_DIM_RED: &str = "\x1b[41m";

/// Set colour of text to green
pub const GREEN: &str = "\x1b[92m";
/// Set background of text to green
pub const BG_GREEN: &str = "\x1b[102m";
/// Set colour of text to dim green
pub const DIM_GREEN: &str = "\x1b[32m";
/// Set background of text to dim green
pub const BG_DIM_GREEN: &str = "\x1b[42m";

/// Set colour of text to yellow
pub const YELLOW: &str = "\x1b[93m";
/// Set background of text to yellow
pub const BG_YELLOW: &str = "\x1b[103m";
/// Set colour of text to dim yellow
pub const DIM_YELLOW: &str = "\x1b[33m";
/// Set background of text to dim yellow
pub const BG_DIM_YELLOW: &str = "\x1b[43m";

/// Set colour of text to blue
pub const BLUE: &str = "\x1b[94m";
/// Set background of text to blue
pub const BG_BLUE: &str = "\x1b[104m";
/// Set colour of text to dim blue
pub const DIM_BLUE: &str = "\x1b[34m";
/// Set background of text to dim blue
pub const BG_DIM_BLUE: &str = "\x1b[44m";

/// Set colour of text to magenta
pub const MAGENTA: &str = "\x1b[95m";
/// Set background of text to magenta
pub const BG_MAGENTA: &str = "\x1b[105m";
/// Set colour of text to dim magenta
pub const DIM_MAGENTA: &str = "\x1b[35m";
/// Set background of text to dim magenta
pub const BG_DIM_MAGENTA: &str = "\x1b[45m";

/// Set colour of text to cyan
pub const CYAN: &str = "\x1b[96m";
/// Set background of text to cyan
pub const BG_CYAN: &str = "\x1b[106m";
/// Set colour of text to dim cyan
pub const DIM_CYAN: &str = "\x1b[36m";
/// Set background of text to dim cyan
pub const BG_DIM_CYAN: &str = "\x1b[46m";

/// Set colour of text to white
pub const WHITE: &str = "\x1b[97m";
/// Set background of text to white
pub const BG_WHITE: &str = "\x1b[107m";
/// Set colour of text to dim white
pub const DIM_WHITE: &str = "\x1b[37m";
/// Set background of text to dim white
pub const BG_DIM_WHITE: &str = "\x1b[47m";

/// Set colour of text to default
pub const DEFAULT: &str = "\x1b[99m";
/// Set background of text to default
pub const BG_DEFAULT: &str = "\x1b[109m";
/// Set colour of text to default
pub const DIM_DEFAULT: &str = "\x1b[39m";
/// Set background of text to default
pub const BG_DIM_DEFAULT: &str = "\x1b[49m";

/// Clear the terminal
pub const CLEAR_TERMINAL: &str = "\x1bc";
/// Clear the previous line
pub const ERASE_LINE_UP: &str = "\x1b[1A\x1b[1G\x1b[0K";
/// Move to line start
pub const LINE_START: &str = "\x1b[1G";
/// Erase the current line
pub const ERASE_LINE: &str = "\x1b[0K";
/// Move to previous line
pub const LINE_UP: &str = "\x1b[1A";

/// Format an error message
pub fn format_error<W: Write, S: Display>(message: S) -> String {
    format!("{BOLD}{RED}error{DEFAULT}:{RESET} {message}")
}

/// Format a warning message
pub fn format_warning<S: Display>(message: S) -> String {
    format!("{BOLD}{YELLOW}warning{DEFAULT}:{RESET} {message}")
}

/// Format a success message
pub fn format_success<S: Display>(message: S) -> String {
    format!("{BOLD}{GREEN}Success{DEFAULT}:{RESET} {message}")
}

/// Format a failure message
pub fn format_failure<S: Display>(message: S) -> String {
    format!("{BOLD}{RED}Failure{DEFAULT}:{RESET} {message}")
}
