use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_bin_keys<K: SortKey>(
        &mut self,
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    );
}

impl<T: Copy + Send> TwoKeysBinSortParallel<T> for [T] {
    fn par_sort_by_two_bin_keys<K: SortKey>(
        &mut self,
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        let cpu = CPUCount::count();
        if let Some(mid_layout) = MidLayout::with_keys(self, key1, cpu) {
            mid_layout.par_sort_by_two_bin_keys(self, key1, key2);
        } else {
            self.sort_by_two_bin_keys(key1, key2);
        }
    }
}
