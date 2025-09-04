use crate::sort::layout::{BinKey, BinKeyFn, BinLayout};

pub trait OneKeyBinSortSerial<T> {
    fn sort_by_one_bin_key<K: BinKey>(&mut self, key: BinKeyFn<T, K>);
    fn sort_by_one_bin_key_and_buffer<K: BinKey>(&mut self, buffer: &mut [T], key: BinKeyFn<T, K>);
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    fn sort_by_one_bin_key<K: BinKey>(&mut self, key: BinKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key(self, key);
    }

    fn sort_by_one_bin_key_and_buffer<K: BinKey>(&mut self, buffer: &mut [T], key: BinKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key_and_buffer(self, buffer, key);
    }
}
