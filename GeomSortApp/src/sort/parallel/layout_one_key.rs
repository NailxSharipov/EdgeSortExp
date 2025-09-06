use crate::sort::key::{KeyFn, SortKey};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::partition::Partition;

const MIN_LEN_PER_TASK: usize = 32_000;

impl<K: SortKey> MidLayout<K> {
    #[inline]
    pub fn par_sort_by_one_key<T: Copy + Send + Default, F: KeyFn<T, K>>(&self, slice: &mut [T], key: F) {
        let (left_layout, right_layout) = if let Some((left, right)) = self.children_layout() {
            (left, right)
        } else {
            slice.sort_by_one_key(key);
            return;
        };

        let md = slice.partition(self.mid_key(), key);

        let (left_slice, right_slice) = slice.split_at_mut(md);

        let is_left_big = left_slice.len() > MIN_LEN_PER_TASK;
        let is_right_big = right_slice.len() > MIN_LEN_PER_TASK;

        if is_left_big && is_right_big {
            rayon::join(
                || left_layout.par_sort_by_one_key(left_slice, key),
                || right_layout.par_sort_by_one_key(right_slice, key),
            );
        } else {
            if is_left_big {
                left_layout.par_sort_by_one_key(left_slice, key)
            } else {
                left_slice.sort_by_one_key(key);
            }

            if is_right_big {
                right_layout.par_sort_by_one_key(right_slice, key)
            } else {
                right_slice.sort_by_one_key(key);
            }
        }
    }
}
