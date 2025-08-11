//! A string value.

use core::{iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;
use unicode_segmentation::UnicodeSegmentation;

/// A string value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue {
    /// The span of the string, including the surrounding quotes.
    pub span: Span,
    /// The value of the string, does not include the quotes.
    pub value: String,
}

impl StringValue {
    /// Parse a string value.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) -> Option<Self> {
        iter.next_if_eq(&'\"')?;

        let mut span = global_span.length(0);
        let mut value = String::new();

        let mut is_escaped = false;

        #[expect(
            clippy::while_let_on_iterator,
            reason = "consistent with other parsers"
        )]
        while let Some(character) = iter.next() {
            if is_escaped {
                value.push(character);
                is_escaped = false;
                continue;
            }

            match character {
                '\"' => {
                    break;
                }
                '\\' => {
                    is_escaped = true;
                    value.push('\\');
                }
                _ => {
                    value.push(character);
                }
            }
        }

        let columns = value.graphemes(true).count() + 2;
        global_span.column += columns;
        span.length = columns;

        Some(Self { span, value })
    }
}

impl core::fmt::Display for StringValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
