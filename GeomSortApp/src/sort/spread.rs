use crate::sort::layout::{BinKey, BinLayout};
use crate::sort::mapper::Mapper;

impl<K: BinKey> BinLayout<K> {

    pub(crate) fn spread_with_buffer<T: Copy, KeyFn>(&self, array: &mut [T], buffer: &mut [T], key: &KeyFn) -> Mapper
    where
        KeyFn: Fn(&T) -> K,
    {

        let mut mapper = Mapper::new(self.count());
        for a in array.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices(array.len());

        for a in array.iter() {
            let index = mapper.next_index(self.index(key(a)));
            unsafe {
                *buffer.get_unchecked_mut(index) = *a;
            }
        }

        array.copy_from_slice(buffer);

        mapper
    }
}