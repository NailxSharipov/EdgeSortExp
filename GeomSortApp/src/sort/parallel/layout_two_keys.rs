use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mid_layout::MidLayout;
use crate::sort::parallel::partition::Partition;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

impl<K: SortKey> MidLayout<K> {
    #[inline]
    pub fn par_sort_by_two_keys<T: Copy + Send + Default, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        key1: F1,
        key2: F2,
    ) {
        let (left_layout, right_layout) = if let Some((left, right)) = self.children_layout() {
            (left, right)
        } else {
            slice.sort_by_two_keys(key1, key2);
            return;
        };

        let md = slice.partition(self.mid_key(), key1);

        let (left_slice, right_slice) = slice.split_at_mut(md);

        let is_left_big = left_slice.len() > MIN_LEN_PER_TASK;
        let is_right_big = right_slice.len() > MIN_LEN_PER_TASK;

        if is_left_big && is_right_big {
            rayon::join(
                || left_layout.par_sort_by_two_keys(left_slice, key1, key2),
                || right_layout.par_sort_by_two_keys(right_slice, key1, key2),
            );
        } else {
            if is_left_big {
                left_layout.par_sort_by_two_keys(left_slice, key1, key2)
            } else {
                left_slice.sort_by_two_keys(key1, key2);
            }

            if is_right_big {
                right_layout.par_sort_by_two_keys(right_slice, key1, key2)
            } else {
                right_slice.sort_by_two_keys(key1, key2);
            }
        }
    }
}
