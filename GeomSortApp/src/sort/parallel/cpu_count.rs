pub(super) struct CPUCount;

impl CPUCount {
    #[inline]
    pub(super) fn count() -> usize {
        match std::thread::available_parallelism() {
            Ok(value) => value.get(),
            Err(_) => 1,
        }
    }
}