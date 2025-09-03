pub trait BinKey: Copy + Ord {
    fn difference(self, other: Self) -> usize;
}

impl BinKey for i32 {
    #[inline(always)]
    fn difference(self, other: Self) -> usize {
        (self - other) as usize
    }
}

pub struct BinLayout<K> {
    pub(crate) min_key: K,
    pub(crate) max_key: K,
    pub(crate) power: usize,
}

pub const MAX_BINS_POWER: u32 = 6;
pub const MAX_BINS_COUNT: usize = 1 << MAX_BINS_POWER;

impl<K> BinLayout<K>
where
    K: BinKey,
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
    pub fn new(min_key: K, max_key: K) -> BinLayout<K> {
        let length = max_key.difference(min_key) + 1;
        let scale = length.ilog2_ceil();
        let power = scale.saturating_sub(MAX_BINS_POWER) as usize;

        Self {
            min_key,
            max_key,
            power,
        }
    }

    #[inline]
    pub fn with_keys<T, KeyFn>(array: &[T], key: &KeyFn) -> Option<Self>
    where
        KeyFn: Fn(&T) -> K,
    {
        let first_val = array.first()?;
        let first_key = key(first_val);

        let mut min_key = first_key;
        let mut max_key = first_key;

        for val in array.iter().skip(1) {
            let k = key(val);
            min_key = min_key.min(k);
            max_key = max_key.max(k);
        }

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
    use crate::sort::layout::BinLayout;

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
