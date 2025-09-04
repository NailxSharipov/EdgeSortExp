use crate::sort::layout::{BinKey, BinKeyFn, BinLayout};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

impl<K: BinKey> BinLayout<K> {
    pub fn par_sort_by_one_bin_key<T: Copy + Send, KeyFn>(&self, slice: &mut [T], key: BinKeyFn<T, K>)
    where
        KeyFn: Fn(&T) -> K + Sync,
    {

        // let mapper = self.spread_with_buffer(slice, key);
        //
        // if mapper.is_final() {
        //     return;
        // }

        slice
            .par_chunk_by_mut(|v0, v1| key(v0) == key(v1))
            .for_each(|s| s.sort_by_one_bin_key(key));
    }

    fn par_sort<T: Copy + Send, KeyFn>(
        &self,
        slice: &mut [T],
        key: &KeyFn,
    ) where
        KeyFn: Fn(&T) -> K + Sync,
    {
        
    }
    
}
