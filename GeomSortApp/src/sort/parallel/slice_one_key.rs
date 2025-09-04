use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>);
}

impl<T: Copy + Send> OneKeyBinSortParallel<T> for [T] {
    fn par_sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>) {
        let cpu = CPUCount::count();
        if let Some(mid_layout) = MidLayout::with_keys(self, key, cpu) {
            mid_layout.par_sort_by_one_key(self, key);
        } else {
            self.sort_by_one_key(key);
        }
    }
}
