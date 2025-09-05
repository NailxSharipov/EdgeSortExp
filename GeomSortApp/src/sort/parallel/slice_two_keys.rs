use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

pub trait TwoKeysBinSortParallel<T> {
    #[inline]
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy + Send> TwoKeysBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        let cpu = CPUCount::count();
        if let Some(mid_layout) = MidLayout::with_keys(self, key1, cpu) {
            mid_layout.par_sort_by_two_keys(self, key1, key2);
        } else {
            self.sort_by_two_keys(key1, key2);
        }
    }
}
