use crate::sort::layout::{BinKey, BinLayout};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

impl<K: BinKey> BinLayout<K> {
    pub fn par_sort_by_one_bin_key<T: Copy + Send, KeyFn>(&self, slice: &mut [T], key: &KeyFn)
    where
        KeyFn: Fn(&T) -> K + Sync,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.par_sort_by_one_bin_key_and_buffer(slice, &mut buffer, key);
    }

    pub fn par_sort_by_one_bin_key_and_buffer<T: Copy + Send, KeyFn>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key: &KeyFn,
    ) where
        KeyFn: Fn(&T) -> K + Sync,
    {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key);

        if mapper.is_final() {
            return;
        }

        slice
            .par_chunk_by_mut(|v0, v1| key(v0) == key(v1))
            .for_each(|s| s.sort_by_one_bin_key(key));
    }
}
