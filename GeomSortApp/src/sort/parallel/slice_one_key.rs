use crate::sort::layout::{BinKey, BinLayout};
use crate::sort::parallel::cpu_count::CpuCount;

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_bin_key<K, KeyFn>(&mut self, key: &KeyFn)
    where
        K: BinKey,
        KeyFn: Fn(&T) -> K + Sync;

    fn par_sort_by_one_bin_key_and_buffer<K: BinKey, KeyFn: Fn(&T) -> K>(
        &mut self,
        buffer: &mut [T],
        key: &KeyFn,
    ) where
        K: BinKey,
        KeyFn: Fn(&T) -> K + Sync;
}

impl<T: Copy + Send> OneKeyBinSortParallel<T> for [T] {
    fn par_sort_by_one_bin_key<K, KeyFn>(&mut self, key: &KeyFn)
    where
        K: BinKey,
        KeyFn: Fn(&T) -> K + Sync {
        let max_bin_power = CpuCount::max_bin_power();
        let layout = if let Some(layout) = BinLayout::with_keys_max_bins(max_bin_power, self, key) {
            layout
        } else {
            return;
        };

        layout.par_sort_by_one_bin_key(self, key);
    }

    fn par_sort_by_one_bin_key_and_buffer<K: BinKey, KeyFn: Fn(&T) -> K>(
        &mut self,
        buffer: &mut [T],
        key: &KeyFn,
    ) where
        K: BinKey,
        KeyFn: Fn(&T) -> K + Sync,
    {
        let max_bin_power = CpuCount::max_bin_power();
        let layout = if let Some(layout) = BinLayout::with_keys_max_bins(max_bin_power, self, key) {
            layout
        } else {
            return;
        };

        layout.par_sort_by_one_bin_key_and_buffer(self, buffer, key);
    }
}
