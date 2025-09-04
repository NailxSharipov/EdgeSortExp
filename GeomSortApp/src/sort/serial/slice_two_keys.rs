use crate::sort::layout::{BinKey, BinKeyFn, BinLayout};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait TwoKeysBinSortSerial<T> {
    fn sort_by_two_bin_keys<K: BinKey>(&mut self, key1: BinKeyFn<T, K>, key2: BinKeyFn<T, K>);

    fn sort_by_two_bin_keys_and_buffer<K: BinKey>(
        &mut self,
        buffer: &mut [T],
        key1: BinKeyFn<T, K>,
        key2: BinKeyFn<T, K>,
    );
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    fn sort_by_two_bin_keys<K: BinKey>(&mut self, key1: BinKeyFn<T, K>, key2: BinKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout
        } else {
            // already sorted by key1
            self.sort_by_one_bin_key(key2);
            return;
        };

        layout.sort_by_two_bin_keys(self, key1, key2);
    }

    fn sort_by_two_bin_keys_and_buffer<K: BinKey>(
        &mut self,
        buffer: &mut [T],
        key1: BinKeyFn<T, K>,
        key2: BinKeyFn<T, K>,
    ) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout
        } else {
            // already sorted by key1
            self.sort_by_one_bin_key(key2);
            return;
        };

        layout.sort_by_two_bin_keys_and_buffer(self, buffer, key1, key2);
    }
}
