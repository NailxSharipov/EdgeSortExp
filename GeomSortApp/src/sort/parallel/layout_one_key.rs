use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::partition::Partition;

const MIN_LEN_PER_TASK: usize = 64_000;

impl<K: SortKey> MidLayout<K> {
    pub fn par_sort_by_one_bin_key<T: Copy + Send>(&self, slice: &mut [T], key: SortKeyFn<T, K>) {
        let (left_layout, right_layout) = if let Some((left, right)) = self.children_layout() {
            (left, right)
        } else {
            slice.sort_by_one_bin_key(key);
            return;
        };

        let (lo, hi) = slice.partition3_by_mid(self.mid_key(), key);

        let (left_slice, slice) = slice.split_at_mut(lo);
        let (_, right_slice) = slice.split_at_mut(hi - lo);

        let is_left_big = left_slice.len() > MIN_LEN_PER_TASK;
        let is_right_big = right_slice.len() > MIN_LEN_PER_TASK;

        if is_left_big && is_right_big {
            rayon::join(
                || left_layout.par_sort_by_one_bin_key(left_slice, key),
                || right_layout.par_sort_by_one_bin_key(right_slice, key),
            );
        } else {
            if is_left_big {
                left_layout.par_sort_by_one_bin_key(left_slice, key)
            } else {
                left_slice.sort_by_one_bin_key(key);
            }

            if is_right_big {
                right_layout.par_sort_by_one_bin_key(right_slice, key)
            } else {
                right_slice.sort_by_one_bin_key(key);
            }
        }
    }
}
