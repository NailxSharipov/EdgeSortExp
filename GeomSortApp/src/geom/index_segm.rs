use crate::geom::segm::Segment;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct IndexSegment { // 24
    pub seg: Segment, // 8 + 8
    pub index: usize  // 8
}

impl IndexSegment {
    #[inline(always)]
    pub fn new(index: usize, s: &Segment) -> Self {
        Self { seg: s.clone(), index }
    }
}