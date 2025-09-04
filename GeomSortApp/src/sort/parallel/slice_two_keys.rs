use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>);
    fn par_sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    );
}

impl<T: Copy + Send> TwoKeysBinSortParallel<T> for [T] {
    fn par_sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>) {
        let cpu = CPUCount::count();
        if let Some(layout) = BinLayout::with_keys_and_cpu(self, key1, cpu) {
            layout.par_sort_by_two_keys(self, key1, key2);
        } else {
            self.par_sort_by_one_key(key2);
        }
    }

    fn par_sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        let cpu = CPUCount::count();
        if let Some(layout) = BinLayout::with_keys_and_cpu(self, key1, cpu) {
            layout.par_sort_by_two_keys_and_buffer(self, buffer, key1, key2);
        } else {
            self.par_sort_by_one_key_and_buffer(buffer, key2);
        }
    }
}
