use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::median::Median;
use crate::sort::parallel::partition::Partition;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

const MIN_LEN_PER_TASK: usize = 64_000;

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy + Send + Default> TwoKeysBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        let cpu = CPUCount::count();
        if cpu == 1 {
            self.sort_by_two_keys(key1, key2);
        } else {
            let level = cpu.ilog2() + 2;
            par_sort_with_level(self, level, key1, key2);
        }
    }
}

fn par_sort_with_level<T: Send + Default + Copy, K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
    slice: &mut [T],
    level: u32,
    key1: F1,
    key2: F2,
) {
    let md_key = if let Some(median) = slice.median(key1) {
        median
    } else {
        slice.sort_by_two_keys(key1, key2);
        return;
    };

    let md = slice.partition(md_key, key1);

    let (left_slice, right_slice) = slice.split_at_mut(md);

    if level == 0 || left_slice.len() < MIN_LEN_PER_TASK || right_slice.len() < MIN_LEN_PER_TASK {
        // switch to serial sort
        let len = left_slice.len().max(right_slice.len());
        let mut buffer = vec![T::default(); len];
        left_slice.sort_by_two_keys_and_buffer(&mut buffer[..left_slice.len()], key1, key2);
        right_slice.sort_by_two_keys_and_buffer(&mut buffer[..right_slice.len()], key1, key2);
    } else {
        rayon::join(
            || par_sort_with_level(left_slice, level - 1, key1, key2),
            || par_sort_with_level(right_slice, level - 1, key1, key2),
        );
    }
}
