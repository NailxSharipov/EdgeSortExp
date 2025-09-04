use crate::sort::layout::{BinKey, BinKeyFn, BinLayout, MIN_BINS_POWER};
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_bin_keys<K: BinKey>(&mut self, key1: BinKeyFn<T, K>, key2: BinKeyFn<T, K>);

    fn par_sort_by_two_bin_keys_and_buffer<K: BinKey>(
        &mut self,
        buffer: &mut [T],
        key1: BinKeyFn<T, K>,
        key2: BinKeyFn<T, K>,
    );
}

impl<T: Copy + Send> TwoKeysBinSortParallel<T> for [T] {
    fn par_sort_by_two_bin_keys<K: BinKey>(&mut self, key1: BinKeyFn<T, K>, key2: BinKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys_max_bins(MIN_BINS_POWER, self, key1) {
            layout
        } else {
            // already sorted by key1
            self.par_sort_by_one_bin_key(key2);
            return;
        };

        layout.par_sort_by_two_bin_keys(self, key1, key2);
    }

    fn par_sort_by_two_bin_keys_and_buffer<K: BinKey>(
        &mut self,
        buffer: &mut [T],
        key1: BinKeyFn<T, K>,
        key2: BinKeyFn<T, K>,
    ) {
        let layout = if let Some(layout) = BinLayout::with_keys_max_bins(MIN_BINS_POWER, self, key1) {
            layout
        } else {
            // already sorted by key1
            self.par_sort_by_one_bin_key(key2);
            return;
        };

        layout.par_sort_by_two_bin_keys_and_buffer(self, buffer, key1, key2);
    }
}
