use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F);
}

impl<T: Copy + Send + Default> OneKeyBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F) {
        let cpu = CPUCount::count();
        if let Some(mid_layout) = MidLayout::with_keys(self, key, cpu) {
            mid_layout.par_sort_by_one_key(self, key);
        } else {
            self.sort_by_one_key(key);
        }
    }
}
