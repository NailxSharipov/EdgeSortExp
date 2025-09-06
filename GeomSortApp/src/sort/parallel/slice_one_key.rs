use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::median::Median;
use crate::sort::parallel::partition::Partition;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

const MIN_LEN_PER_TASK: usize = 64_000;

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F);
}

impl<T: Copy + Send + Default> OneKeyBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F) {
        let cpu = CPUCount::count();
        if cpu == 1 {
            self.sort_by_one_key(key);
        } else {
            let level = cpu.ilog2() + 2;
            par_sort_with_level(self, level, key);
        }
    }
}

fn par_sort_with_level<T: Send + Default + Copy, K: SortKey, F: KeyFn<T, K>>(slice: &mut [T], level: u32, key: F) {
    let md_key = if let Some(median) = slice.median(key) {
        median
    } else {
        slice.sort_by_one_key(key);
        return;
    };

    let md = slice.partition(md_key, key);

    let (left_slice, right_slice) = slice.split_at_mut(md);

    if level == 0 || left_slice.len() < MIN_LEN_PER_TASK || right_slice.len() < MIN_LEN_PER_TASK {
        // switch to serial sort
        let len = left_slice.len().max(right_slice.len());
        let mut buffer = vec![T::default(); len];
        left_slice.sort_by_one_key_and_buffer(&mut buffer[..left_slice.len()], key);
        right_slice.sort_by_one_key_and_buffer(&mut buffer[..right_slice.len()], key);
    } else {
        rayon::join(
            || par_sort_with_level(left_slice, level - 1, key),
            || par_sort_with_level(right_slice, level - 1, key),
        );
    }
}

