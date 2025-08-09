#[derive(Clone, Debug)]
/// A simple cursor over a slice.
pub struct Cursor<'a, T: Copy + Default> {
    index: usize,
    collection: &'a [T],
}

impl<T: Copy + Default> core::fmt::Display for Cursor<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "index {} of {}", self.index, self.collection.len())
    }
}

impl<'a, T: Copy + Default> Cursor<'a, T> {
    /// Pull some items from this source into the specified buffer, returning how many items were
    /// read.
    ///
    /// If this function returns `0` then either:
    /// 1. The `buffer` is of length zero.
    /// 2. All items have been read from the source.
    ///
    /// # Panics
    /// * If it attempts to read out of bounds, which should only happen if the implementation is
    ///   incorrect.
    pub fn read(&mut self, buffer: &mut [T]) -> usize {
        let item_count = buffer.len().min(self.collection.len() - self.index);
        if item_count == 0 {
            return 0;
        }
        let data = self
            .read_count(item_count)
            .expect("read should never read out of bounds");
        buffer
            .get_mut(..item_count)
            .expect("read should never read out of bounds")
            .copy_from_slice(data);
        item_count
    }

    /// Pull exactly `N` items from the source into an array.
    pub fn read_array<const N: usize>(&mut self) -> Result<[T; N], OutOfBounds> {
        let mut output = [T::default(); N];
        let data = self.read_count(N)?;
        output.copy_from_slice(data);
        Ok(output)
    }

    /// Pull the next `count` items from the source.
    pub fn read_count<N: Into<usize>>(&mut self, count: N) -> Result<&[T], OutOfBounds> {
        let count = count.into();
        let data = self
            .collection
            .get(self.index..self.index + count)
            .ok_or_else(|| OutOfBounds::new(count))?;
        self.index += count;

        Ok(data)
    }
}

impl std::io::Read for Cursor<'_, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(self.read(buf))
    }
}

/// A read would take the cursor out of bounds.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct OutOfBounds {
    requested: usize,
}
impl OutOfBounds {
    fn new(requested: usize) -> Self {
        Self { requested }
    }
}
impl core::fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "reading {} bytes would take the cursor out of bounds",
            self.requested,
        )
    }
}
impl core::error::Error for OutOfBounds {}
