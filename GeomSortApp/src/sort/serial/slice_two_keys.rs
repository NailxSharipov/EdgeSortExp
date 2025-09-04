use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

pub trait TwoKeysBinSortSerial<T> {
    fn sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>);

    fn sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    );
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    fn sort_by_two_keys<K: SortKey>(&mut self, key1: SortKeyFn<T, K>, key2: SortKeyFn<T, K>) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys(self, key1, key2);
        } else {
            // already sorted by key1
            self.sort_by_one_key(key2);
        }
    }

    fn sort_by_two_keys_and_buffer<K: SortKey>(
        &mut self,
        buffer: &mut [T],
        key1: SortKeyFn<T, K>,
        key2: SortKeyFn<T, K>,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_buffer(self, buffer, key1, key2);
        } else {
            // already sorted by key1
            self.sort_by_one_key(key2);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::point::Point;
    use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;
    use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

    #[test]
    fn test_0() {
        let x = [1, 2, 1000, 1000, 1000, 10, 10];
        let y = [2, 2, 3, 4, 5, 6, 7];

        let pts: Vec<_> = x
            .iter()
            .zip(y.iter())
            .map(|(&x, &y)| Point::new(x, y))
            .collect();

        let mut pts_1 = pts.clone();
        pts_1.sort_by_two_keys(|p| p.x, |p| p.y);

        let mut pts_2 = pts.to_vec();
        pts_2.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        assert_eq!(pts_1, pts_2);
    }

    #[test]
    fn test_1() {
        let x: Vec<_> = (0..100000).into_iter().rev().collect();
        let y: Vec<_> = x.iter().map(|x| x / 2048).collect();

        let pts: Vec<_> = x
            .iter()
            .zip(y.iter())
            .map(|(&x, &y)| Point::new(x, y))
            .collect();

        let mut pts_1 = pts.clone();
        pts_1.sort_by_two_keys(|p| p.x, |p| p.y);

        let mut pts_2 = pts.clone();
        pts_2.par_sort_by_two_keys(|p| p.x, |p| p.y);

        let mut pts_3 = pts.to_vec();
        pts_3.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        assert_eq!(pts_1, pts_3);
        assert_eq!(pts_2, pts_3);
    }
}
