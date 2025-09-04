use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::bin_layout::BinLayout;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

impl<K: SortKey> BinLayout<K> {

    pub fn sort_by_two_keys<T: Copy>(&self, slice: &mut [T], key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>) {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe { buffer.set_len(slice.len()); }
        self.sort_by_two_keys_and_buffer(slice, &mut buffer, key1, key2);
    }

    pub fn sort_by_two_keys_and_buffer<T: Copy>(&self, slice: &mut [T], buffer: &mut [T], key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>) {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key1);

        if self.one_to_one() {
            return;
        }

        for range in mapper.iter_ranges() {
            if range.len() < 2 { continue; }
            let (sub_slice, sub_buffer) = unsafe {
                let sub_buffer = buffer.get_unchecked_mut(0..range.len());
                let sub_slice = slice.get_unchecked_mut(range);
                (sub_slice, sub_buffer)
            };

            sub_slice.sort_by_two_keys_and_buffer(sub_buffer, key1, key2);
        }
    }
}

