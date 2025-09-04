use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::parallel::cpu_count::CPUCount;

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>);
    fn par_sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>);
}

impl<T: Copy + Send> OneKeyBinSortParallel<T> for [T] {
    fn par_sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>) {
        let cpu = CPUCount::count();
        if let Some(layout) = BinLayout::with_keys_and_cpu(self, key, cpu) {
            layout.par_sort_by_one_key(self, key);
        };
    }

    fn par_sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>) {
        let cpu = CPUCount::count();
        if let Some(layout) = BinLayout::with_keys_and_cpu(self, key, cpu) {
            layout.par_sort_by_one_key_and_buffer(self, buffer, key);
        };
    }
}
