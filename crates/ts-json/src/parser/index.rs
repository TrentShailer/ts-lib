use jsonschema::paths::{Location, LocationSegment};

use crate::parser::{Node, Value};

pub(crate) enum Index<'a> {
    Tag(&'a str),
    Index(usize),
}

impl Node {
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

impl Value {
    /// Try index the node.
    pub fn get<'a, 'b>(&'b self, index: Index<'a>) -> Option<&'b Node> {
        match &self {
            Self::Object(properties) => {
                if let Index::Tag(index_tag) = index {
                    properties.iter().find(|property| {
                        property
                            .tag
                            .as_ref()
                            .is_some_and(|tag| tag.value == index_tag)
                    })
                } else {
                    None
                }
            }
            Self::Array(items) => {
                if let Index::Index(index) = index {
                    items.get(index)
                } else {
                    None
                }
            }
            Self::Value(_) => None,
        }
    }
}
