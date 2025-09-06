use crate::sort::key::KeyFn;

const MIN_LEN_SLICE_LEN: usize = 32_000;

pub(super) trait Median<T> {
    fn median<K: Ord + Copy, F: KeyFn<T, K>>(&self, key: F) -> Option<K>;
}

impl<T> Median<T> for [T] {

    #[inline]
    fn median<K: Ord + Copy, F: KeyFn<T, K>>(&self, key: F) -> Option<K> {
        let n = self.len();
        if n < MIN_LEN_SLICE_LEN {
            return None;
        }

        let s = n / 8;

        let i0 = 0;
        let i1 = 1 * s + 17;
        let i2 = 2 * s + 37;
        let i3 = 3 * s + 101;
        let i4 = 4 * s + 301;
        let i5 = 5 * s - 301;
        let i6 = 6 * s - 101;
        let i7 = 7 * s - 37;
        let i8 = n - 1;

        let k0 = key(&self[i0]);
        let k1 = key(&self[i1]);
        let k2 = key(&self[i2]);
        let k3 = key(&self[i3]);
        let k4 = key(&self[i4]);
        let k5 = key(&self[i5]);
        let k6 = key(&self[i6]);
        let k7 = key(&self[i7]);
        let k8 = key(&self[i8]);

        let mut keys = [k0, k1, k2, k3, k4, k5, k6, k7, k8];
        keys.sort();

        if keys[0].eq(&keys[8]) {
            return None
        }

        let mk = &keys[keys.len() / 2];
        if keys[0].ne(mk) && keys[8].ne(mk) {
            Some(*mk)
        } else {
            None
        }
    }
}