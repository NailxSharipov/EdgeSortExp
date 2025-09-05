use crate::sort::bin_layout::MAX_BINS_COUNT;
use std::ops::Range;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Chunk {
    pub(crate) index: usize,
    pub(crate) count: usize,
}

pub struct Mapper {
    pub(crate) count: usize,
    pub(crate) chunks: [Chunk; MAX_BINS_COUNT],
}

impl Mapper {
    #[inline(always)]
    pub(crate) fn new(count: usize) -> Self {
        debug_assert!(count <= MAX_BINS_COUNT);
        Self {
            count,
            chunks: [Chunk::default(); MAX_BINS_COUNT],
        }
    }

    #[inline(always)]
    pub(super) fn inc_bin_count(&mut self, chunk_index: usize) {
        let chunk = unsafe { self.chunks.get_unchecked_mut(chunk_index) };
        chunk.count += 1;
    }

    #[inline(always)]
    pub(super) fn next_index(&mut self, chunk_index: usize) -> usize {
        let chunk = unsafe { self.chunks.get_unchecked_mut(chunk_index) };
        let index = chunk.index;
        chunk.index += 1;
        index
    }

    #[inline(always)]
    pub(super) fn init_indices(&mut self) {
        let mut offset = 0;
        for chunk in self.chunks[..self.count].iter_mut() {
            chunk.index = offset;
            offset += chunk.count;
        }
    }
}

impl Chunk {
    #[inline(always)]
    pub(crate) fn to_range(&self) -> Range<usize> {
        let end = self.index;
        let start = end - self.count;
        start..end
    }
}