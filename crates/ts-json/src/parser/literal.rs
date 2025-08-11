//! A literal value, such as a number, boolean, or null.

use core::{iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;

/// A literal value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    /// The span of the literal.
    pub span: Span,
    /// The value of the literal.
    pub value: String,
}
impl Literal {
    /// Parse a literal.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) -> Option<Self> {
        let mut span = global_span.length(0);
        let mut value = String::new();

        while let Some(character) = iter.next_if(|character| {
            "null".contains(*character)
                || "false".contains(*character)
                || "true".contains(*character)
                || character.is_ascii_digit()
                || matches!(character, '.' | '+' | 'e' | 'E' | '-')
        }) {
            value.push(character);
            span.length += 1;
            global_span.column += 1;
        }

        if value.is_empty() {
            return None;
        }

        Some(Self { span, value })
    }
}

impl core::fmt::Display for Literal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.value)
    }
}
