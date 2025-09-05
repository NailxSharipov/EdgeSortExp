pub trait KeyFn<T, K>: Fn(&T) -> K + Send + Sync + Copy {

}
impl<T, K, F: Fn(&T) -> K + Send + Sync + Copy> KeyFn<T, K> for F {

}

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
