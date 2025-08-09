#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A span for diagnostics, maps to a location in a source file.
pub struct Span {
    /// One-indexed line number.
    pub line: usize,
    /// One-indexed column of the span start.
    pub column: usize,
    /// Number of characters the span goes for.
    pub length: usize,
}
impl Default for Span {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            length: 1,
        }
    }
}
impl Span {
    /// Sets the line of the span, lines should be one-indexed.
    pub fn line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    /// Sets the column of the span, columns should be one-indexed.
    pub fn column(mut self, column: usize) -> Self {
        self.column = column;
        self
    }

    /// Sets the length of the span.
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
}
