use core::{fmt::Write, iter::Peekable};

use ts_error::diagnostic::Span;

use crate::parser::{Tag, increment_span, parse_string};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Node {
    pub tag: Option<Tag>,
    pub value_span: Span,
    pub value: Value,
}

impl Node {
    pub fn parse_document(src: &str) -> Option<Self> {
        let mut global_span = Span::default();
        let mut iter = src.chars().peekable();

        Self::parse(&mut global_span, &mut iter)
    }

    fn parse<I: Iterator<Item = char>>(
        global_span: &mut Span,
        iter: &mut Peekable<I>,
    ) -> Option<Self> {
        let mut tag = None;
        let mut value = None;

        while let Some(character) = iter.peek() {
            match character {
                character if character.is_whitespace() => {
                    let character = iter.next().expect("if peek() succeeds, next() should");
                    increment_span(global_span, character);
                }
                '"' => {
                    if tag.is_none() {
                        tag = Tag::parse(global_span, iter);
                    } else {
                        value = Value::parse_value(global_span, iter);
                        break;
                    }
                }
                '{' => {
                    value = Value::parse_object(global_span, iter);
                    break;
                }
                '[' => {
                    value = Value::parse_array(global_span, iter);
                    break;
                }
                _ => {
                    value = Value::parse_value(global_span, iter);
                    break;
                }
            }
        }

        let (value_span, value) = value?;
        Some(Self {
            tag,
            value_span,
            value,
        })
    }
}

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(tag) = &self.tag {
            write!(f, "{tag}")?;
        }

        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Value {
    #[expect(clippy::enum_variant_names, reason = "JSON naming")]
    Value(String),
    Object(Vec<Node>),
    Array(Vec<Node>),
}

impl Value {
    fn parse_value<I: Iterator<Item = char>>(
        global_span: &mut Span,
        iter: &mut Peekable<I>,
    ) -> Option<(Span, Self)> {
        if iter.peek()? == &'"' {
            let (span, value) = parse_string(global_span, iter)?;
            Some((span, Self::Value(value)))
        } else {
            let mut span = global_span.length(0);
            let mut value = String::with_capacity(16);

            while let Some(character) = iter.peek() {
                match character {
                    character
                        if character.is_whitespace()
                            || character.is_control()
                            || matches!(character, ',' | '}' | ']') =>
                    {
                        break;
                    }

                    _ => {
                        let character =
                            iter.next().expect("if peek() succeeds, then next() should");
                        increment_span(global_span, character);
                        span.length += 1;
                        value.push(character);
                    }
                }
            }

            Some((span, Self::Value(value)))
        }
    }

    fn parse_array<I: Iterator<Item = char>>(
        global_span: &mut Span,
        iter: &mut Peekable<I>,
    ) -> Option<(Span, Self)> {
        let character = iter.next_if_eq(&'[')?;
        let span = global_span.length(1);
        increment_span(global_span, character);

        let mut items: Vec<Node> = vec![];

        while let Some(character) = iter.peek() {
            match character {
                character if character.is_whitespace() || character == &',' => {
                    let character = iter
                        .next()
                        .expect("next() should succeed if peek() succeeds");
                    increment_span(global_span, character);
                }
                '{' => {
                    if let Some((span, object)) = Self::parse_object(global_span, iter) {
                        items.push(Node {
                            tag: None,
                            value_span: span,
                            value: object,
                        });
                    }
                }
                '[' => {
                    if let Some((span, array)) = Self::parse_array(global_span, iter) {
                        items.push(Node {
                            tag: None,
                            value_span: span,
                            value: array,
                        });
                    }
                }
                ']' => {
                    let character = iter
                        .next()
                        .expect("next() should succeed if peek() succeeds");
                    increment_span(global_span, character);
                    break;
                }
                _ => {
                    if let Some((span, value)) = Self::parse_value(global_span, iter) {
                        items.push(Node {
                            tag: None,
                            value_span: span,
                            value,
                        });
                    }
                }
            }
        }

        Some((span, Self::Array(items)))
    }

    fn parse_object<I: Iterator<Item = char>>(
        global_span: &mut Span,
        iter: &mut Peekable<I>,
    ) -> Option<(Span, Self)> {
        let character = iter.next_if_eq(&'{')?;
        let span = global_span.length(1);
        increment_span(global_span, character);

        let mut properties: Vec<Node> = vec![];

        while let Some(character) = iter.peek() {
            match character {
                '"' => {
                    if let Some(property) = Node::parse(global_span, iter) {
                        properties.push(property);
                    }
                }
                '}' => {
                    let character = iter
                        .next()
                        .expect("next() should succeed if peek() succeeds");
                    increment_span(global_span, character);
                    break;
                }
                _ => {
                    let character = iter
                        .next()
                        .expect("next() should succeed if peek() succeeds");
                    increment_span(global_span, character);
                }
            }
        }

        Some((span, Self::Object(properties)))
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Array(items) => {
                f.write_char('[')?;
                for (index, item) in items.iter().enumerate() {
                    write!(f, "{item}")?;
                    if index != items.len() - 1 {
                        f.write_char(',')?;
                    }
                }
                f.write_char(']')?;
            }
            Self::Object(properties) => {
                f.write_char('[')?;
                for (index, property) in properties.iter().enumerate() {
                    write!(f, "{property}")?;
                    if index != properties.len() - 1 {
                        f.write_char(',')?;
                    }
                }
                f.write_char(']')?;
            }
            Self::Value(value) => write!(f, "{value}")?,
        }

        Ok(())
    }
}
