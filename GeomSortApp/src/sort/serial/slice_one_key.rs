use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::bin_layout::BinLayout;

pub trait OneKeyBinSortSerial<T> {
    fn sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>);
    fn sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>);
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    fn sort_by_one_key<K: SortKey>(&mut self, key: SortKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_key(self, key);
    }

    fn sort_by_one_key_and_buffer<K: SortKey>(&mut self, buffer: &mut [T], key: SortKeyFn<T, K>) {
        let layout = if let Some(layout) = BinLayout::with_keys(self, key) {
            layout
        } else {
            return;
        };

        layout.sort_by_one_key_and_buffer(self, buffer, key);
    }
}


#[cfg(test)]
mod tests {
    use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
    use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

    #[test]
    fn test_0() {
        let num = [12, 102, 1003, 10004, 1005, 106, 17];
        let mut num_1 = num.to_vec();
        num_1.sort_by_one_key(|&n|n);

        let mut num_2 = num.to_vec();
        num_2.sort();

        assert_eq!(num_1, num_2);
    }

    #[test]
    fn test_1() {
        let num: Vec<_> = (0..100000).into_iter().rev().collect();
        let mut num_1 = num.to_vec();
        num_1.sort_by_one_key(|&n|n);

        let mut num_2 = num.to_vec();
        num_2.par_sort_by_one_key(|&n|n);

        let mut num_3 = num.to_vec();
        num_3.sort();

        assert_eq!(num_1, num_3);
        assert_eq!(num_1, num_3);
    }
}