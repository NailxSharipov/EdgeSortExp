use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::bin_layout::BinLayout;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait TwoKeysBinSortSerial<T> {
    fn sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>);

    fn sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    );
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    fn sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys(self, key1, key2);
        } else {
            // already sorted by key1
            self.sort_by_one_key(key2);
        };
    }

    fn sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_buffer(self, buffer, key1, key2);
        } else {
            // already sorted by key1
            self.sort_by_one_key_and_buffer(buffer, key2);
        }
    }
}
