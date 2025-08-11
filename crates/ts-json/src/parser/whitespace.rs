//! JSON whitespace.

use core::{iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;

/// Whitespace in a JSON document.
pub struct Whitespace;
impl Whitespace {
    /// Parse some whitespace, updating the global span line and column.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) {
        let mut previous_was_newline = false;
        while let Some(character) =
            iter.next_if(|character| matches!(character, ' ' | '\n' | '\r' | '\t'))
        {
            match character {
                '\n' | '\r' => {
                    if !previous_was_newline {
                        previous_was_newline = true;
                        global_span.column = 1;
                        global_span.line += 1;
                    }
                }
                _ => {
                    previous_was_newline = false;
                    global_span.column += 1;
                }
            }
        }
    }
}
