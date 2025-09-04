use std::ops::Range;
use crate::sort::bin_layout::MAX_BINS_COUNT;

#[derive(Debug, Clone, Copy, Default)]
struct Chunk {
    index: usize,
    count: usize
}

pub struct Mapper {
    count: usize,
    chunks: [Chunk; MAX_BINS_COUNT]
}

impl Mapper {

    #[inline(always)]
    pub(super) fn new(count: usize) -> Self {
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
        for chunk in self.chunks.iter_mut() {
            chunk.index = offset;
            offset += chunk.count;
        }
    }

    #[inline]
    pub(crate) fn to_ends(&self) -> Vec<usize> {
        debug_assert!(self.chunks.len().is_power_of_two());
        let mut result = Vec::with_capacity(self.count);
        for ch in self.chunks[..self.count].iter() {
            result.push(ch.index);
        }
        result
    }
}

pub struct ChunkRanges<'a> {
    iter: std::slice::Iter<'a, Chunk>,
}

impl<'a> Iterator for ChunkRanges<'a> {
    type Item = Range<usize>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(chunk) = self.iter.next() {
            if chunk.count > 0 {
                let end = chunk.index;
                let start = end - chunk.count;
                return Some(start..end);
            }
        }
        None
    }
}

impl Mapper {
    #[inline(always)]
    pub(super) fn iter_ranges(&self) -> ChunkRanges<'_> {
        ChunkRanges {
            iter: self.chunks.iter(),
        }
    }
}


