use crate::sort::key::KeyFn;

const MIN_LEN_SLICE_LEN: usize = 32_000;

trait Median<T> {
    fn median<K: Ord, F: KeyFn<T, K>>(&self, key: F) -> Option<K>;
}

impl<T> Median<T> for [T] {

    #[inline]
    fn median<K: Ord, F: KeyFn<T, K>>(&self, key: F) -> Option<K> {
        let n = self.len();
        if n < MIN_LEN_SLICE_LEN {
            return None;
        }
        let k0 = key(&self[0]);
        let k1 = key(&self[n / 4]);
        let k2 = key(&self[n / 2]);
        let k3 = key(&self[3 * n / 4]);

        let mut keys = [k0, k1, k2, k3];
        keys.sort();


        None
    }
}