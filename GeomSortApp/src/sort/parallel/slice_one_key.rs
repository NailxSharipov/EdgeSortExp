use crate::sort::layout::{BinKey, BinKeyFn};

pub trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_bin_key<K: BinKey>(&mut self, key: BinKeyFn<T, K>);
}

impl<T: Copy + Send> OneKeyBinSortParallel<T> for [T] {
    fn par_sort_by_one_bin_key<K: BinKey>(&mut self, key: BinKeyFn<T, K>) {



    }
}
