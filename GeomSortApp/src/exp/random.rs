use rand::Rng;
use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;
use crate::solver::sort::SortSolution;

pub struct RandomTest {
    segments: Vec<Segment>,
    index_segments: Vec<IndexSegment>,
    id_segments: Vec<IdSegment>,
}

impl RandomTest {

    pub fn new(n: usize) -> Self {
        let segments = Self::checkerboard_segments(20, 30, n);
        let index_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IndexSegment::new(i, s)).collect();
        let id_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IdSegment::new(i, s)).collect();
        Self {
            segments,
            index_segments,
            id_segments,
        }
    }

    pub fn run_all(&self) {
        println!("Checkerboard");
        self.run_segments();
        println!();
        self.run_index_segments();
        println!();
        self.run_id_segments();
    }

    pub fn run_segments(&self) {
        println!("Segments test n = {}", self.segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.segments);
        SortSolution::run_segments_sort_unstable(&self.segments);
        SortSolution::run_segments_par_sort_stable(&self.segments);
        SortSolution::run_segments_par_sort_unstable(&self.segments);
        SortSolution::run_segments_bin_sort(&self.segments);
    }

    pub fn run_index_segments(&self) {
        println!("Index Segments test n = {}", self.index_segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.index_segments);
        SortSolution::run_segments_sort_unstable(&self.index_segments);
        SortSolution::run_segments_par_sort_stable(&self.index_segments);
        SortSolution::run_segments_par_sort_unstable(&self.index_segments);
        SortSolution::run_segments_bin_sort(&self.index_segments);
    }

    pub fn run_id_segments(&self) {
        println!("Id Segments test n = {}", self.id_segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.id_segments);
        SortSolution::run_segments_sort_unstable(&self.id_segments);
        SortSolution::run_segments_par_sort_stable(&self.id_segments);
        SortSolution::run_segments_par_sort_unstable(&self.id_segments);
        SortSolution::run_segments_bin_sort(&self.id_segments);
    }

    fn checkerboard_segments(size: i32, offset: i32, n: usize) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(n * n);
        let start = Point::new(0, 0);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let p0 = Point::new(x, y);
                let p1 = Point::new(x, y + size);
                let p2 = Point::new(x + size, y + size);
                let p3 = Point::new(x + size, y);

                let s0 = Segment::new(p0, p1);
                let s1 = Segment::new(p1, p2);
                let s2 = Segment::new(p2, p3);
                let s3 = Segment::new(p3, p0);

                vec.push(s0);
                vec.push(s1);
                vec.push(s2);
                vec.push(s3);

                x += offset;
            }
            y += offset;
        }

        vec
    }

    fn random_segments(n: usize, min: i32, max: i32, len: i32) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(n);
        let mut rng = rand::rng();

        for _ in 0..n {
            let x0 = rng.random_range(min..max);
            let y0 = rng.random_range(min..max);
            let dx = rng.random_range(0..len);
            let dy = rng.random_range(0..len);

            let x1 = x0 + dx;
            let y1 = y0 + dy;

            let a = Point::new(x0, y0);
            let b = Point::new(x1, y1);

            vec.push(Segment::new(a, b));
        }

        vec
    }
}
