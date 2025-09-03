use crate::sort::layout::{BinKey, BinLayout};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

impl<K: BinKey> BinLayout<K> {
    pub fn par_sort_by_two_bin_keys<T: Copy + Send, KeyFn1, KeyFn2>(&self, slice: &mut [T], key1: &KeyFn1, key2: &KeyFn2)
    where
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.par_sort_by_two_bin_keys_and_buffer(slice, &mut buffer, key1, key2);
    }

    pub fn par_sort_by_two_bin_keys_and_buffer<T: Copy + Send, KeyFn1, KeyFn2>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key1: &KeyFn1,
        key2: &KeyFn2
    ) where
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync,
    {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key1);

        if mapper.is_final() {
            return;
        }

        slice
            .par_chunk_by_mut(|v0, v1| key1(v0) == key1(v1))
            .for_each(|s| s.sort_by_two_bin_keys(key1, key2));
    }
}
