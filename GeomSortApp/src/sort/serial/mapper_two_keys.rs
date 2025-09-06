use std::cmp::Ordering;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_two_keys<K: SortKey, T: Copy + Default, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key1: F1,
        key2: F2,
    ) {
        for chunk in self.chunks[..self.count].iter() {
            match chunk.count() {
                0..=1 => {
                    continue;
                }
                2..=63 => {
                    let sub_slice = unsafe { slice.get_unchecked_mut(chunk.as_range()) };
                    sub_slice.sort_unstable_by(|a, b| {
                        let ordering = key1(a).cmp(&key1(b));
                        if ordering == Ordering::Equal {
                            key2(a).cmp(&key2(b))
                        } else {
                            ordering
                        }
                    });
                }
                _ => {
                    let range = chunk.as_range();
                    let (sub_slice, sub_buffer) = unsafe {
                        let sub_buffer = buffer.get_unchecked_mut(..range.len());
                        let sub_slice = slice.get_unchecked_mut(range);
                        (sub_slice, sub_buffer)
                    };

                    sub_slice.sort_by_two_keys_and_buffer(sub_buffer, key1, key2);
                }
            }
        }
    }
}