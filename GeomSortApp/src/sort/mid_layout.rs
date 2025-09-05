use crate::sort::bin_layout::MIN_BINS_POWER;
use crate::sort::key::{SortKey, SortKeyFn};
use crate::sort::min_max::MinMax;

pub(crate) struct MidLayout<K> {
    mid_key: K,
    min_key: K,
    max_key: K,
    level: u32,
}

impl<K> MidLayout<K>
where
    K: SortKey,
{
    #[inline(always)]
    pub(crate) fn mid_key(&self) -> K {
        self.mid_key
    }

    #[inline(always)]
    fn new(min_key: K, max_key: K, level: u32) -> Self {
        let mid_key = min_key.middle(max_key);
        Self {
            mid_key,
            min_key,
            max_key,
            level,
        }
    }

    #[inline(always)]
    pub(crate) fn children_layout(&self) -> Option<(Self, Self)> {
        if self.level == 0 {
            return None;
        }
        let child_level = self.level - 1;

        let left  = Self::new(self.min_key, self.mid_key, child_level); // [min, mid)
        let right = Self::new(self.mid_key, self.max_key, child_level); // [mid, max]

        Some((left, right))
    }

    #[inline]
    pub(crate) fn with_keys<T>(array: &[T], key: SortKeyFn<T, K>, cpu: usize) -> Option<Self> {
        if array.is_empty() || cpu == 1 {
            return None;
        }

        let (min_key, max_key) = array.min_max(key);
        let span = max_key.difference(min_key);
        if span == 0 {
            // all keys equal
            return None;
        }

        let possible_by_cpu = cpu.ilog2() + 3;
        let required_by_bins = span.ilog2().saturating_sub(MIN_BINS_POWER);
        let level = required_by_bins.min(possible_by_cpu);

        if level == 0 {
            // split strategy is not optimal
            None
        } else {
            Some(Self::new(min_key, max_key, level))
        }
    }
}
