use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub fn sort_by_one_key<T: Copy + Default, F: KeyFn<T, K>>(&self, slice: &mut [T], key: F) {
        let mut buffer: Vec<T> = vec![T::default(); slice.len()];
        self.sort_by_one_key_and_buffer(slice, &mut buffer, key);
    }

    #[inline]
    pub fn sort_by_one_key_and_buffer<T: Copy + Default, F: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key: F,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key);

        if !self.one_to_one() {
            mapper.sort_chunks_by_one_key(slice, buffer, key);
        }
    }
}
