use crate::sort::layout::{BinLayout, BinKey};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

impl<K: BinKey> BinLayout<K> {

    pub fn sort_by_one_bin_key<T: Copy, KeyFn>(&self, slice: &mut [T], key: &KeyFn)
    where
        KeyFn: Fn(&T) -> K,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(slice.len());
        unsafe { buffer.set_len(slice.len()); }
        self.sort_by_one_bin_key_and_buffer(slice, &mut buffer, key);
    }

    pub fn sort_by_one_bin_key_and_buffer<T: Copy, KeyFn>(&self, slice: &mut [T], buffer: &mut [T], key: &KeyFn)
    where
        KeyFn: Fn(&T) -> K,
    {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key);
        if mapper.is_final() {
            return;
        }

        for range in mapper.iter_ranges() {
            if range.len() < 2 { continue; }
            let (sub_slice, sub_buffer) = unsafe {
                let sub_buffer = buffer.get_unchecked_mut(0..range.len());
                let sub_slice = slice.get_unchecked_mut(range);
                (sub_slice, sub_buffer)
            };

            sub_slice.sort_by_one_bin_key_and_buffer(sub_buffer, key);
        }
    }
}

