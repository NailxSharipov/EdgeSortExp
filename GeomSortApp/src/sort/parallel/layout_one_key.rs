use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

const MIN_LEN_PER_TASK: usize = 64_000;

impl<K: SortKey> BinLayout<K> {
    pub fn par_sort_by_one_key<T: Copy + Send>(&self, slice: &mut [T], key: SortKeyFn<T, K>) {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.par_sort_by_one_key_and_buffer(slice, &mut buffer, key);
    }

    pub fn par_sort_by_one_key_and_buffer<T: Copy + Send>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key: SortKeyFn<T, K>,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());
        let mapper = self.spread_with_buffer(slice, buffer, key);

        let mut ends = mapper.to_ends();

        Self::par_sort_by_one_bin_keys_and_ends(slice, &mut ends, 0, key);
    }

    fn par_sort_by_one_bin_keys_and_ends<T: Copy + Send>(
        slice: &mut [T],
        ends: &mut [usize],
        base: usize,
        key: SortKeyFn<T, K>,
    ) {
        if ends.len() < 2 {
            slice.sort_by_one_key(key);
            return;
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
                || Self::par_sort_by_one_bin_keys_and_ends(left_slice, left_ends, left_base, key),
                || {
                    Self::par_sort_by_one_bin_keys_and_ends(
                        right_slice,
                        right_ends,
                        right_base,
                        key,
                    )
                },
            );
        } else {
            if is_left_big {
                left_slice.par_sort_by_one_key(key);
            } else {
                left_slice.sort_by_one_key(key);
            }

            if is_right_big {
                right_slice.par_sort_by_one_key(key);
            } else {
                right_slice.sort_by_one_key(key);
            }
        }
    }
}
