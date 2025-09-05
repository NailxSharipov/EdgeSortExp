use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;

impl<K: SortKey> BinLayout<K> {
    #[inline(always)]
    pub(crate) fn spread_with_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [T],
        key: F,
    ) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in slice.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();

        for a in slice.iter() {
            let index = mapper.next_index(self.index(key(a)));
            unsafe {
                *buffer.get_unchecked_mut(index) = *a;
            }
        }

        slice.copy_from_slice(buffer);

        mapper
    }
}
