use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;
use std::time::Instant;
use rayon::prelude::ParallelSliceMut;
use crate::geom::start_segment::StartEnd;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

pub struct SortSolution;

impl SortSolution {
    pub fn run_segments_sort_unstable<S: StartEnd>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));

        Self::print_result("sort_unstable", data.last().unwrap().end().x, start);
    }

    pub fn run_segments_sort_stable<S: StartEnd>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.sort_by(|s0, s1| s0.cmp_by_start(s1));

        Self::print_result("sort_stable", data.last().unwrap().end().x, start);
    }

    pub fn run_segments_par_sort_unstable<S: StartEnd>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.par_sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));

        Self::print_result("par_sort_unstable", data.last().unwrap().end().x, start);
    }

    pub fn run_segments_par_sort_stable<S: StartEnd>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.par_sort_by(|s0, s1| s0.cmp_by_start(s1));

        Self::print_result("par_sort_stable", data.last().unwrap().end().x, start);
    }

    pub fn run_segments_bin_sort<S: StartEnd + Copy>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.sort_by_two_keys(|s| s.start().x, |s| s.start().y);

        Self::print_result("bin_sort", data.last().unwrap().end().x, start);
    }

    pub fn run_segments_par_bin_sort<S: StartEnd + Copy>(segments: &[S]) {
        let start = Instant::now();

        let mut data = segments.to_vec();
        data.par_sort_by_two_keys(|s| s.start().x, |s| s.start().y);

        Self::print_result("par bin_sort", data.last().unwrap().end().x, start);
    }

    fn print_result(title: &str, result: i32, start: Instant) {
        let duration = start.elapsed().as_secs_f64();
        println!("{} - {:.6} hash: {}", title, duration, result);
    }
}