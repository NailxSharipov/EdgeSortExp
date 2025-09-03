use crate::sort::layout::{BinKey, BinLayout};

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_bin_key<K: BinKey, KeyFn: Fn(&T) -> K>(&mut self, key: &KeyFn);
    fn par_sort_by_one_bin_key_and_buffer<K: BinKey, KeyFn: Fn(&T) -> K>(
        &mut self,
        buffer: &mut [T],
        key: &mut KeyFn,
    );
}

impl<T: Copy> OneKeyBinSortParallel<T> for [T] {
    fn par_sort_by_one_bin_key<K, KeyFn>(&mut self, key: &KeyFn)
    where
        K: BinKey,
        KeyFn: Fn(&T) -> K,
    {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key(self, key);
    }

    fn par_sort_by_one_bin_key_and_buffer<K: BinKey, KeyFn: Fn(&T) -> K>(
        &mut self,
        buffer: &mut [T],
        key: &mut KeyFn,
    ) where
        K: BinKey,
        KeyFn: Fn(&T) -> K,
    {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_bin_key_and_buffer(self, buffer, key);
    }
}
