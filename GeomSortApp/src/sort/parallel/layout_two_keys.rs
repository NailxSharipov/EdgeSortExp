use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

const MIN_LEN_PER_TASK: usize = 256_000;

impl<K: SortKey> BinLayout<K> {
    pub fn par_sort_by_two_keys<T: Copy + Send>(
        &self,
        slice: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.par_sort_by_two_keys_and_buffer(slice, &mut buffer, key1, key2);
    }

    pub fn par_sort_by_two_keys_and_buffer<T: Copy + Send>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());
        let mapper = self.spread_with_buffer(slice, buffer, key1);
        if self.power == 0 {
            return;
        }

        let mut ends = mapper.to_ends();

        Self::par_sort_by_two_keys_and_ends(slice, &mut ends, 0, key1, key2);
    }

    fn par_sort_by_two_keys_and_ends<T: Copy + Send>(
        slice: &mut [T],
        ends: &mut [usize],
        base: usize,
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        if ends.len() < 2 {
            slice.sort_by_two_keys(key1, key2);
            return
        }

        let mid = (ends.len() / 2) - 1;
        let mid_end = ends[mid] - base;
        let (left_slice, right_slice) = slice.split_at_mut(mid_end);

        let is_left_big = left_slice.len() > MIN_LEN_PER_TASK;
        let is_right_big = right_slice.len() > MIN_LEN_PER_TASK;

        if is_left_big && is_right_big {
            let (left_ends, right_ends) = ends.split_at_mut(mid);
            let left_base = base;
            let right_base = base + left_slice.len();

            rayon::join(
                || {
                    Self::par_sort_by_two_keys_and_ends(
                        left_slice, left_ends, left_base, key1, key2,
                    )
                },
                || {
                    Self::par_sort_by_two_keys_and_ends(
                        right_slice,
                        right_ends,
                        right_base,
                        key1,
                        key2,
                    )
                },
            );
        } else {
            if is_left_big {
                left_slice.par_sort_by_two_keys(key1, key2);
            } else {
                left_slice.sort_by_two_keys(key1, key2);
            }

            if is_right_big {
                right_slice.par_sort_by_two_keys(key1, key2);
            } else {
                right_slice.sort_by_two_keys(key1, key2);
            }
        }
    }
}
