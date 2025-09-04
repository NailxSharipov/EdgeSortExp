use crate::sort::key::{SortKey, SortKeyFn};
use std::cmp::Ordering;

pub(super) trait Partition<T> {
    fn partition3_by_mid<K: SortKey>(
        &mut self,
        mid_key: K,
        key: SortKeyFn<T, K>,
    ) -> (usize, usize);
}

impl<T> Partition<T> for [T] {
    #[inline(always)]
    fn partition3_by_mid<K: SortKey>(
        &mut self,
        mid_key: K,
        key: SortKeyFn<T, K>,
    ) -> (usize, usize) {
        let (mut lo, mut i, mut hi) = (0, 0, self.len());
        while i < hi {
            let k = key(&self[i]);
            match k.cmp(&mid_key) {
                Ordering::Less => {
                    self.swap(lo, i);
                    lo += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    hi -= 1;
                    self.swap(i, hi);
                    // i stays; new a[i] must be examined
                }
                Ordering::Equal => {
                    i += 1;
                }
            }
        }
        // now: [0..lo) < mid, [lo..hi) == mid, [hi..len) > mid
        (lo, hi)
    }
}
