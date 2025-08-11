//! A JSON value.

use core::{iter::Peekable, str::Chars};

use ts_error::diagnostic::Span;

use crate::parser::{Array, Index, Literal, Node, Object, StringValue, Whitespace};

/// A JSON value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A string.
    String(StringValue),
    /// A literal.
    Literal(Literal),
    /// An object.
    Object(Object),
    /// An array.
    Array(Array),
}

impl Value {
    /// Parse a value.
    pub fn parse(global_span: &mut Span, iter: &mut Peekable<Chars<'_>>) -> Option<Self> {
        Whitespace::parse(global_span, iter);

        let value = match iter.peek()? {
            '\"' => Self::String(StringValue::parse(global_span, iter)?),
            '{' => Self::Object(Object::parse(global_span, iter)?),
            '[' => Self::Array(Array::parse(global_span, iter)?),
            _ => Self::Literal(Literal::parse(global_span, iter)?),
        };

        Whitespace::parse(global_span, iter);

        Some(value)
    }

    /// Index a value.
    pub fn get<'a, 'b>(&'b self, index: Index<'a>) -> Option<&'b Node> {
        match &self {
            Self::Object(object) => {
                if let Index::Tag(index_tag) = index {
                    object.properties.iter().find(|property| {
                        property
                            .tag
                            .as_ref()
                            .is_some_and(|tag| tag.value == index_tag)
                    })
                } else {
                    None
                }
            }
            Self::Array(array) => {
                if let Index::Index(index) = index {
                    array.items.get(index)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the span of the value.
    pub fn span(&self) -> Span {
        match &self {
            Self::String(string_value) => string_value.span,
            Self::Literal(literal) => literal.span,
            Self::Object(object) => object.span,
            Self::Array(array) => array.span,
        }
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::String(v) => v.fmt(f),
            Self::Literal(v) => v.fmt(f),
            Self::Object(v) => v.fmt(f),
            Self::Array(v) => v.fmt(f),
        }
    }
}
