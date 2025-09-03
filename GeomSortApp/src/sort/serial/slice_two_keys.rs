use crate::sort::layout::{BinKey, BinLayout};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait TwoKeysBinSortSerial<T> {
    fn sort_by_two_bin_keys<K, KeyFn1, KeyFn2>(&mut self, key1: &KeyFn1, key2: &KeyFn2)
    where
        K: BinKey,
        KeyFn1: Fn(&T) -> K,
        KeyFn2: Fn(&T) -> K;

    fn sort_by_two_bin_keys_and_buffer<K, KeyFn1, KeyFn2>(
        &mut self,
        buffer: &mut [T],
        key1: &KeyFn1,
        key2: &KeyFn2,
    ) where
        K: BinKey,
        KeyFn1: Fn(&T) -> K,
        KeyFn2: Fn(&T) -> K;
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    fn sort_by_two_bin_keys<K, KeyFn1, KeyFn2>(&mut self, key1: &KeyFn1, key2: &KeyFn2)
    where
        K: BinKey,
        KeyFn1: Fn(&T) -> K,
        KeyFn2: Fn(&T) -> K,
    {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout
        } else {
            // already sorted by key1
            self.sort_by_one_bin_key(key2);
            return;
        };

        layout.sort_by_two_bin_keys(self, key1, key2);
    }

    fn sort_by_two_bin_keys_and_buffer<K, KeyFn1, KeyFn2>(
        &mut self,
        buffer: &mut [T],
        key1: &KeyFn1,
        key2: &KeyFn2,
    ) where
        K: BinKey,
        KeyFn1: Fn(&T) -> K,
        KeyFn2: Fn(&T) -> K,
    {
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
