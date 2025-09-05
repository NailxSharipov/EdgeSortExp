use std::ptr;
use crate::sort::key::{KeyFn, SortKey};

pub(super) trait Partition<T> {
    fn partition<K: SortKey, F: KeyFn<T, K>>(&mut self, mid_key: K, key: F) -> usize;
}

impl<T> Partition<T> for [T] {
    #[inline(always)]
    fn partition<K: SortKey, F: KeyFn<T, K>>(&mut self, mid_key: K, key: F) -> usize {
        let len = self.len();
        let mut i = 0usize;
        let mut j = len; // exclusive

        // SAFETY:
        // - `ptr` points to `self`'s contiguous storage.
        // - We only form mutable references for indices in-bounds.
        // - At each swap we ensure distinct indices (i < j), so no aliasing of &mut.
        // - Loop maintains 0 <= i <= j <= len.
        let ptr = self.as_mut_ptr();
        unsafe {
            while i < j {
                // advance i while <= mid_key
                while i < j && key(&*ptr.add(i)) <= mid_key {
                    i += 1;
                }
                // retreat j while > mid_key
                while i < j && key(&*ptr.add(j - 1)) > mid_key {
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                // swap the mismatched pair
                ptr::swap(ptr.add(i), ptr.add(j - 1));
                i += 1;
                j -= 1;
            }
        }

        j
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::parallel::partition::Partition;
    #[test]
    fn test_0() {
        let mut arr = vec![5, 3, 1];

        let x = arr.partition(2, |&a| a);

        assert_eq!(x, 1);
    }
}
