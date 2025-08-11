//! # `ts-json`
//!
//! JSON schema validation and reporting

mod location;
mod parser;
mod problem_message;

use std::path::Path;

use jsonschema::ValidationOptions;
use serde_json::Value;
use ts_error::{
    diagnostic::{Context, Diagnostic, Diagnostics},
    normalize_message,
};

use crate::{
    location::LocationExtensions,
    parser::{Node, Value as SpannedValue},
    problem_message::ProblemMessage,
};

/// Error variants for validating JSON.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ValidationError {
    #[non_exhaustive]
    ParseSource { source: serde_json::Error },

    #[non_exhaustive]
    ParseSchema { source: serde_json::Error },

    #[non_exhaustive]
    CreateValidator {
        source: Box<jsonschema::ValidationError<'static>>,
    },
}
impl core::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::ParseSource { .. } => write!(f, "source file is not valid JSON"),
            Self::ParseSchema { .. } => write!(f, "schema is not valid JSON"),
            Self::CreateValidator { .. } => write!(f, "could not create validator from schema"),
        }
    }
}
impl core::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ParseSource { source, .. } | Self::ParseSchema { source, .. } => Some(source),
            Self::CreateValidator { source, .. } => Some(source),
        }
    }
}

/// Validate some JSON against a JSON schema, returning all problems.
pub fn validate(
    source: &str,
    schema: &str,
    source_path: Option<&Path>,
) -> Result<Diagnostics, ValidationError> {
    let source_node: Value =
        serde_json::from_str(source).map_err(|source| ValidationError::ParseSource { source })?;
    let schema_node: Value =
        serde_json::from_str(schema).map_err(|source| ValidationError::ParseSchema { source })?;

    let validator = ValidationOptions::default()
        .build(&schema_node)
        .map_err(|source| ValidationError::CreateValidator {
            source: Box::new(source),
        })?;

    let mut diagnostics = Diagnostics::new("validating JSON");

    if !validator.is_valid(&source_node) {
        let document = Node::parse_document(source);
        for error in validator.iter_errors(&source_node) {
            let context = document.as_ref().and_then(|document| {
                let span = document
                    .evaluate(&error.instance_path)
                    .map(|node| match node.value {
                        SpannedValue::Array(_) | SpannedValue::Object(_) => {
                            if let Some(tag) = &node.tag {
                                tag.span
                            } else {
                                node.value.span()
                            }
                        }
                        _ => node.value.span(),
                    });

                span.map(|span| {
                    let mut context = Context::new(source, span);
                    context.label = error.kind.message();
                    context
                })
            });

            let mut diagnostic = Diagnostic::error(error.kind.headline());
            // TODO headline needs the node

            diagnostic.context = context;
            diagnostic.file_path = source_path.map(|path| path.display().to_string());

            if let Some(parent) = error.schema_path.parent()
                && let Some(node) = schema_node.pointer(parent.join("description").as_str())
                && let Some(contents) = node.as_str()
            {
                for line in contents.lines() {
                    diagnostic.notes.push(normalize_message(line));
                }
            }

            diagnostics.push(diagnostic);
        }
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    const SOURCE: &str = include_str!("../tests/sample.json");
    const SCHEMA: &str = include_str!("../tests/sample.schema.json");

    #[test]
    fn validates_sample_correctly() {
        let diagnostics = crate::validate(
            SOURCE,
            SCHEMA,
            Some(Path::new("crates/ts-json/tests/sample.json")),
        )
        .expect("validation to succeed");
        assert!(!diagnostics.is_empty());
        assert_eq!(4, diagnostics.errors().count());
        eprintln!("{diagnostics}");
    }
}
