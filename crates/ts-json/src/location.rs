//! Extensions to a [`jsonschema::paths::Location`].

use jsonschema::paths::Location;

/// Extension trait to get the parent of a JSON pointer.
pub(crate) trait LocationExtensions: Sized {
    /// Return the pointers parent there is one.
    fn parent(&self) -> Option<Self>;
}

impl LocationExtensions for Location {
    fn parent(&self) -> Option<Self> {
        let mut segments: Vec<_> = self.into_iter().collect();
        if segments.is_empty() {
            return None;
        }
        segments.pop();
        Some(Self::from_iter(segments))
    }
}
