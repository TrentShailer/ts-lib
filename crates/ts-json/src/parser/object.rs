//! A JSON object.

use core::{fmt::Write, iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;

use crate::parser::{Node, StringValue, Value, Whitespace};

/// A JSON object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    /// The span of the opening brace.
    pub span: Span,
    /// The child properties of the object.
    pub properties: Vec<Node>,
}

impl Object {
    /// Parse an object.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) -> Option<Self> {
        iter.next_if_eq(&'{')?;
        let span = global_span.length(1);
        global_span.column += 1;

        let mut properties = Vec::new();

        while iter.peek().is_some_and(|character| *character != '}') {
            Whitespace::parse(global_span, iter);

            let tag = StringValue::parse(global_span, iter)?;

            Whitespace::parse(global_span, iter);
            iter.next_if_eq(&':')?;
            global_span.column += 1;

            if let Some(value) = Value::parse(global_span, iter) {
                properties.push(Node {
                    tag: Some(tag),
                    value,
                });
            };

            if iter.next_if_eq(&',').is_some() {
                global_span.column += 1;
            }
        }

        iter.next_if_eq(&'}')?;
        global_span.column += 1;

        Some(Self { span, properties })
    }
}

impl core::fmt::Display for Object {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('{')?;

        for (index, property) in self.properties.iter().enumerate() {
            write!(f, "{property}")?;
            if index != self.properties.len() - 1 {
                f.write_char(',')?;
            }
        }
        f.write_char('}')?;

        Ok(())
    }
}
