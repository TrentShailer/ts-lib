//! An Array JSON value.

use core::{fmt::Write, iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;

use crate::parser::{Node, Value, Whitespace};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An array JSON value.
pub struct Array {
    /// The span of the opening bracket.
    pub span: Span,
    /// The items in the array.
    pub items: Vec<Node>,
}

impl Array {
    /// Parse an array.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) -> Option<Self> {
        iter.next_if_eq(&'[')?;
        let span = global_span.length(1);
        global_span.column += 1;

        let mut items = Vec::new();
        while iter.peek().is_some_and(|character| *character != ']') {
            Whitespace::parse(global_span, iter);

            if let Some(value) = Value::parse(global_span, iter) {
                items.push(Node { tag: None, value });
            };

            if iter.next_if_eq(&',').is_some() {
                global_span.column += 1;
            }
        }

        iter.next_if_eq(&']')?;
        global_span.column += 1;

        Some(Self { span, items })
    }
}

impl core::fmt::Display for Array {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('[')?;

        for (index, item) in self.items.iter().enumerate() {
            write!(f, "{item}")?;
            if index != self.items.len() - 1 {
                f.write_char(',')?;
            }
        }

        f.write_char(']')?;

        Ok(())
    }
}
