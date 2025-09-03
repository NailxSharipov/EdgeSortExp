use crate::sort::layout::{BinKey, BinLayout};
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_bin_keys<K, KeyFn1, KeyFn2>(&mut self, key1: &KeyFn1, key2: &KeyFn2)
    where
        K: BinKey,
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync;

    fn par_sort_by_two_bin_keys_and_buffer<K, KeyFn1, KeyFn2>(
        &mut self,
        buffer: &mut [T],
        key1: &KeyFn1,
        key2: &KeyFn2,
    ) where
        K: BinKey,
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync;
}

impl<T: Copy + Send> TwoKeysBinSortParallel<T> for [T] {
    fn par_sort_by_two_bin_keys<K, KeyFn1, KeyFn2>(&mut self, key1: &KeyFn1, key2: &KeyFn2)
    where
        K: BinKey,
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync,
    {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout
        } else {
            // already sorted by key1
            self.par_sort_by_one_bin_key(key2);
            return;
        };

        layout.par_sort_by_two_bin_keys(self, key1, key2);
    }

    fn par_sort_by_two_bin_keys_and_buffer<K, KeyFn1, KeyFn2>(
        &mut self,
        buffer: &mut [T],
        key1: &KeyFn1,
        key2: &KeyFn2,
    ) where
        K: BinKey,
        KeyFn1: Fn(&T) -> K + Sync,
        KeyFn2: Fn(&T) -> K + Sync,
    {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout
        } else {
            // already sorted by key1
            self.par_sort_by_one_bin_key(key2);
            return;
        };

        layout.par_sort_by_two_bin_keys_and_buffer(self, buffer, key1, key2);
    }
}
