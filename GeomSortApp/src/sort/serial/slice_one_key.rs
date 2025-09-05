use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

pub trait OneKeyBinSortSerial<T> {
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F);
    fn sort_by_one_key_and_buffer<K: SortKey, F: KeyFn<T, K>>(&mut self, buffer: &mut [T], key: F);
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    #[inline]
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key(self, key);
        }
    }

    #[inline]
    fn sort_by_one_key_and_buffer<K: SortKey, F: KeyFn<T, K>>(&mut self, buffer: &mut [T], key: F) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key_and_buffer(self, buffer, key);
        }
    }
}
