use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::bin_layout::BinLayout;

pub trait OneKeyBinSortSerial<T> {
    fn sort_by_one_bin_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>);
    fn sort_by_one_bin_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>);
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    fn sort_by_one_bin_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key(self, key);
    }

    fn sort_by_one_bin_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key_and_buffer(self, buffer, key);
    }
}
