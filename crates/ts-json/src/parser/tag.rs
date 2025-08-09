use core::iter::Peekable;

use ts_error::diagnostic::Span;

use crate::parser::{increment_span, parse_string};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tag {
    pub span: Span,
    pub value: String,
}
impl Tag {
    pub fn parse<I: Iterator<Item = char>>(
        global_span: &mut Span,
        iter: &mut Peekable<I>,
    ) -> Option<Self> {
        let (span, value) = parse_string(global_span, iter)?;

        // Consume `(\s|:)*`
        while let Some(character) = iter.peek() {
            match character {
                character if character.is_whitespace() || character == &':' => {
                    let character = iter.next().expect("if peek() succeeds, next() should");
                    increment_span(global_span, character);
                }
                _ => break,
            }
        }

        Some(Self { span, value })
    }
}
impl core::fmt::Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"{}\": ", self.value)
    }
}
