pub type SortKeyFn<T, K> = fn(&T) -> K;

pub trait SortKey: Copy + Ord + Sync {
    fn difference(self, other: Self) -> usize;
    fn middle(self, other: Self) -> Self;
}

impl SortKey for i32 {
    #[inline(always)]
    fn difference(self, other: Self) -> usize {
        (self - other) as usize
    }
    #[inline(always)]
    fn middle(self, other: Self) -> Self {
        self + ((other - self) >> 1)
    }
}
