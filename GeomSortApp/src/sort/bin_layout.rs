use crate::sort::key::{KeyFn, SortKey};
use crate::sort::min_max::MinMax;

pub struct BinLayout<K> {
    pub(crate) min_key: K,
    pub(crate) max_key: K,
    pub(crate) power: usize,
    one_to_one: bool,
}

pub const MAX_BINS_POWER: u32 = 8;
pub const MIN_BINS_POWER: u32 = 6;
pub const MAX_BINS_COUNT: usize = 1 << MAX_BINS_POWER;

impl<K> BinLayout<K>
where
    K: SortKey,
{
    #[inline(always)]
    pub(super) fn one_to_one(&self) -> bool {
        self.one_to_one
    }

    #[inline(always)]
    pub fn index(&self, value: K) -> usize {
        let offset = value.difference(self.min_key);
        offset >> self.power
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.index(self.max_key) + 1
    }

    #[inline(always)]
    fn new(min_key: K, max_key: K) -> BinLayout<K> {
        let length = max_key.difference(min_key);
        if length < MAX_BINS_COUNT {
            return Self {
                min_key,
                max_key,
                power: 0,
                one_to_one: true,
            };
        }

        let scale = (length + 1).ilog2_ceil();
        let power = scale.saturating_sub(MAX_BINS_POWER) as usize;

        Self {
            min_key,
            max_key,
            power,
            one_to_one: false,
        }
    }

    #[inline(always)]
    pub fn with_keys<T, F: KeyFn<T, K>>(array: &[T], key: F) -> Option<Self> {
        if array.is_empty() {
            return None;
        }

        let (min_key, max_key) = array.min_max(key);

        if min_key == max_key {
            return None;
        }

        Some(Self::new(min_key, max_key))
    }
}

trait Log2 {
    fn ilog2_ceil(&self) -> u32;
}

impl Log2 for usize {
    #[inline(always)]
    fn ilog2_ceil(&self) -> u32 {
        let floor = self.ilog2();
        if self.is_power_of_two() {
            floor
        } else {
            floor + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bin_layout::BinLayout;

    #[test]
    fn test_0() {
        let layout = BinLayout::<i32>::new(0i32, 3i32);
        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_1() {
        let layout = BinLayout::<i32>::new(0, 255);

        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_2() {
        let layout = BinLayout::<i32>::new(0, 256);

        assert_eq!(layout.power, 1);
    }
}
