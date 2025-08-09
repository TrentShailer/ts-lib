mod index;
mod node;
mod tag;

use core::iter::Peekable;

use ts_error::diagnostic::Span;

pub(crate) use node::{Node, Value};
pub(crate) use tag::Tag;

fn parse_string<I: Iterator<Item = char>>(
    global_span: &mut Span,
    iter: &mut Peekable<I>,
) -> Option<(Span, String)> {
    let character = iter.next_if_eq(&'"')?;
    let mut span = global_span.length(1);
    increment_span(global_span, character);

    let mut value = String::with_capacity(16);
    value.push(character);

    let mut is_escaped = false;
    for character in iter.by_ref() {
        increment_span(global_span, character);
        span.length += 1;

        if is_escaped {
            value.push(character);
            is_escaped = false;
            continue;
        }

        match character {
            '\\' => {
                value.push('\\');
                is_escaped = true;
            }
            '"' => {
                value.push(character);
                break;
            }
            _ => value.push(character),
        }
    }

    value.shrink_to_fit();
    Some((span, value))
}

fn increment_span(span: &mut Span, character: char) {
    match character {
        '\n' => {
            span.line += 1;
            span.column = 1;
        }
        character if character.is_control() => {}
        _ => span.column += 1,
    }
}

#[cfg(test)]
mod test {
    use ts_error::diagnostic::Span;

    use crate::parser::{Node, Tag, Value};

    const SAMPLE: &str = include_str!("../../tests/sample.json");

    #[test]
    fn handles_sample() {
        let expected = Node {
            tag: None,
            value_span: Span::default(),
            value: Value::Object(vec![Node {
                tag: Some(Tag {
                    span: Span::default().line(2).column(3).length(11),
                    value: r#""locations""#.to_string(),
                }),
                value_span: Span::default().line(2).column(16),
                value: Value::Array(vec![
                    Node {
                        tag: None,
                        value_span: Span::default().line(3).column(5),
                        value: Value::Object(vec![
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(4).length(6),
                                    value: r#""name""#.to_string(),
                                }),
                                value_span: Span::default().column(15).line(4).length(13),
                                value: Value::Value(r#""location\nA""#.to_string()),
                            },
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(5).length(8),
                                    value: r#""isCool""#.to_string(),
                                }),
                                value_span: Span::default().column(17).line(5).length(4),
                                value: Value::Value("true".to_string()),
                            },
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(6).length(13),
                                    value: r#""coordinates""#.to_string(),
                                }),
                                value_span: Span::default().column(22).line(6).length(1),
                                value: Value::Array(vec![
                                    Node {
                                        tag: None,
                                        value_span: Span::default().column(23).line(6).length(7),
                                        value: Value::Value("12.0235".to_string()),
                                    },
                                    Node {
                                        tag: None,
                                        value_span: Span::default().column(32).line(6).length(8),
                                        value: Value::Value("-43.0435".to_string()),
                                    },
                                ]),
                            },
                        ]),
                    },
                    Node {
                        tag: None,
                        value_span: Span::default().line(8).column(5),
                        value: Value::Object(vec![
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(9).length(6),
                                    value: r#""name""#.to_string(),
                                }),
                                value_span: Span::default().column(15).line(9).length(15),
                                value: Value::Value(r#""location\"B\"""#.to_string()),
                            },
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(10).length(8),
                                    value: r#""isCool""#.to_string(),
                                }),
                                value_span: Span::default().column(17).line(10).length(5),
                                value: Value::Value("false".to_string()),
                            },
                            Node {
                                tag: Some(Tag {
                                    span: Span::default().column(7).line(11).length(13),
                                    value: r#""coordinates""#.to_string(),
                                }),
                                value_span: Span::default().column(22).line(11).length(1),
                                value: Value::Array(vec![
                                    Node {
                                        tag: None,
                                        value_span: Span::default().column(23).line(11).length(7),
                                        value: Value::Value("12.0235".to_string()),
                                    },
                                    Node {
                                        tag: None,
                                        value_span: Span::default().column(32).line(11).length(4),
                                        value: Value::Value("null".to_string()),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
            }]),
        };

        let document = Node::parse_document(SAMPLE).expect("document should parse");
        assert_eq!(expected, document);
    }
}
