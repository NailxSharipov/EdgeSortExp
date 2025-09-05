use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};

pub trait OneKeyBinSortSerial<T> {
    fn sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>);
    fn sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>);
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    #[inline]
    fn sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key(self, key);
        }
    }

    #[inline]
    fn sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key_and_buffer(self, buffer, key);
        }
    }
}
