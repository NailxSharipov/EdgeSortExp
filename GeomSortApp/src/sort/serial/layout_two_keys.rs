use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};

impl<K: SortKey> BinLayout<K> {
    pub fn sort_by_two_keys<T: Copy>(
        &self,
        slice: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.sort_by_two_keys_and_buffer(slice, &mut buffer, key1, key2);
    }

    pub fn sort_by_two_keys_and_buffer<T: Copy>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key1);

        if self.one_to_one() {
            mapper.sort_chunks_by_one_key(slice, buffer, key2);
        } else {
            mapper.sort_chunks_by_two_keys(slice, buffer, key1, key2);
        }
    }
}
