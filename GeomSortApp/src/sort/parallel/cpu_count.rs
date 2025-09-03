pub(super) struct CpuCount;

impl CpuCount {

    #[inline]
    fn count() -> usize {
        match std::thread::available_parallelism() {
            Ok(value) => value.get(),
            Err(_) => 1,
        }
    }

    #[inline]
    pub(super) fn max_bin_power() -> u32 {
        let n = Self::count();
        (2 * n).ilog2()
    }
}