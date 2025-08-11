//! A JSON parser that tracks the line, column, and length of nodes for diagnostics. This is a loose
//! implementation of <https://www.json.org/json-en.html>, and is not designed for deserialization
//! nor validation of a JSON document. Additionally, this implementation should be able to correctly
//! track the line, column, and length of tags and string values when containing Unicode.

mod array;
mod literal;
mod object;
mod string;
mod value;
mod whitespace;

use jsonschema::paths::{Location, LocationSegment};
use ts_error::diagnostic::Span;

pub(crate) use array::Array;
pub(crate) use literal::Literal;
pub(crate) use object::Object;
pub(crate) use string::StringValue;
pub(crate) use value::Value;
pub(crate) use whitespace::Whitespace;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A JSON node, optional tag and a value.
pub struct Node {
    /// The node's tag, root nodes and array items do not have tags.
    pub tag: Option<StringValue>,
    /// The value of the node.
    pub value: Value,
}

impl Node {
    /// Try parse a source document.
    pub fn parse_document(source: &str) -> Option<Self> {
        let mut global_span = Span::default();
        let mut iter = source.chars().peekable();

        let value = Value::parse(&mut global_span, &mut iter)?;

        Some(Self { tag: None, value })
    }

    /// Try evaluate a pointer to the node it is pointing at.
    pub fn evaluate(&self, pointer: &Location) -> Option<&Self> {
        let segments = pointer.into_iter();

        let mut current_node = self;
        for segment in segments {
            match segment {
                LocationSegment::Property(tag) => {
                    current_node = current_node.get(Index::Tag(tag))?;
                }
                LocationSegment::Index(index) => {
                    current_node = current_node.get(Index::Index(index))?;
                }
            }
        }

        Some(current_node)
    }

    /// Try index the node.
    pub fn get<'a, 'b>(&'b self, index: Index<'a>) -> Option<&'b Self> {
        self.value.get(index)
    }
}

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(tag) = &self.tag {
            write!(f, "{tag}: ")?;
        }

        write!(f, "{}", self.value)
    }
}

/// An index into a JSON structure.
pub(crate) enum Index<'a> {
    /// Index an object by tag.
    Tag(&'a str),
    /// Index an array by index.
    Index(usize),
}

#[cfg(test)]
mod test {
    use ts_error::diagnostic::Span;

    use crate::parser::{Array, Literal, Node, Object, StringValue, Value};

    const SAMPLE: &str = include_str!("../../tests/sample.json");

    fn object(span: Span, properties: Vec<Node>) -> Value {
        Value::Object(Object { span, properties })
    }

    fn array(span: Span, items: Vec<Value>) -> Value {
        Value::Array(Array {
            span,
            items: items
                .into_iter()
                .map(|value| Node { tag: None, value })
                .collect(),
        })
    }

    fn string<S: ToString>(span: Span, value: S) -> Value {
        Value::String(StringValue {
            span,
            value: value.to_string(),
        })
    }

    fn literal<S: ToString>(span: Span, value: S) -> Value {
        Value::Literal(Literal {
            span,
            value: value.to_string(),
        })
    }

    fn node<S: ToString>(tag_span: Span, tag: S, value: Value) -> Node {
        Node {
            tag: Some(StringValue {
                span: tag_span,
                value: tag.to_string(),
            }),
            value,
        }
    }

    #[test]
    fn handles_sample() {
        let expected = object(
            Span::default(),
            vec![node(
                Span::default().line(2).column(3).length(7),
                "array",
                array(
                    Span::default().line(2).column(12),
                    vec![
                        object(
                            Span::default().line(3).column(5),
                            vec![
                                node(
                                    Span::default().line(4).column(7).length(6),
                                    "text",
                                    string(
                                        Span::default().column(15).line(4).length(18),
                                        "다람쥐 헌\\n 쳇바퀴에 타고파",
                                    ),
                                ),
                                node(
                                    Span::default().line(5).column(7).length(6),
                                    "flag",
                                    literal(Span::default().column(15).line(5).length(5), "false"),
                                ),
                                node(
                                    Span::default().line(6).column(7).length(8),
                                    "number",
                                    literal(
                                        Span::default().line(6).column(17).length(7),
                                        "-1.04e2",
                                    ),
                                ),
                            ],
                        ),
                        object(
                            Span::default().line(8).column(5),
                            vec![
                                node(
                                    Span::default().line(9).column(7).length(6),
                                    "text",
                                    string(
                                        Span::default().line(9).column(15).length(41),
                                        "\\\"ابجد هوز حطي كلمن سعفص قرشت ثخذ ضظغ\\\"",
                                    ),
                                ),
                                node(
                                    Span::default().line(10).column(7).length(6),
                                    "flag",
                                    literal(Span::default().line(10).column(15).length(4), "true"),
                                ),
                                node(
                                    Span::default().line(11).column(7).length(8),
                                    "number",
                                    literal(Span::default().line(11).column(17).length(4), "null"),
                                ),
                            ],
                        ),
                    ],
                ),
            )],
        );

        let document = Node::parse_document(SAMPLE).expect("document should parse");
        assert_eq!(expected, document.value);
    }
}
