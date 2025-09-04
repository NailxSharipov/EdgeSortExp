use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::min_max::MinMax;

pub struct BinLayout<K> {
    pub(crate) min_key: K,
    pub(crate) max_key: K,
    pub(crate) power: usize,
}

pub const MAX_BINS_POWER: u32 = 8;
pub const MIN_BINS_POWER: u32 = 6;
pub const MAX_BINS_COUNT: usize = 1 << MAX_BINS_POWER;

impl<K> BinLayout<K>
where
    K: SortKey,
{
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
    fn new(min_key: K, max_key: K, max_bins_power: u32) -> BinLayout<K> {
        let length = max_key.difference(min_key) + 1;
        let scale = length.ilog2_ceil();

        let power = scale.saturating_sub(max_bins_power) as usize;

        Self {
            min_key,
            max_key,
            power,
        }
    }

    #[inline]
    pub fn with_keys_and_cpu<T>(
        array: &[T],
        key: SortKeyFn<T, K>,
        cpu: usize,
    ) -> Option<Self> {
        if array.is_empty() {
            return None;
        }

        let (min_key, max_key) = array.min_max(key);

        if min_key == max_key {
            return None;
        }

        let possible_by_cpu = if cpu > 1 {
            (4 * cpu - 1).ilog2()
        } else {
            MAX_BINS_POWER
        };

        let possible_by_count = array.len().ilog2_ceil();
        let possible = possible_by_count.min(possible_by_cpu);

        Some(Self::new(min_key, max_key, possible))
    }

    #[inline]
    pub fn with_keys<T>(array: &[T], key: fn(&T) -> K) -> Option<Self> {
        Self::with_keys_and_cpu(array, key, 0)
    }
}

trait Log2 {
    fn ilog2_ceil(&self) -> u32;
}

impl Log2 for usize {
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
        let layout = BinLayout::<i32>::new(0i32, 3i32, 8);

        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_1() {
        let layout = BinLayout::<i32>::new(0, 255, 8);

        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_2() {
        let layout = BinLayout::<i32>::new(0, 256, 8);

        assert_eq!(layout.power, 1);
    }
}
