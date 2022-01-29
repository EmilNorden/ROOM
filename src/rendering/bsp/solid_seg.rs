use crate::rendering::View;

const MAXSEGS: usize = 32;

#[derive(Copy, Clone)]
pub struct ClipRange {
    pub first: i32,
    pub last: i32,
}

impl Default for ClipRange {
    fn default() -> Self {
        Self { first: 0, last: 0 }
    }
}

pub struct SolidSegs {
    pub segs: [ClipRange; MAXSEGS],
    pub valid_segs_count: usize,
}

impl SolidSegs {
    pub fn new(view: &View) -> Self {
        let mut segs = [ClipRange::default(); MAXSEGS];
        segs[0].first = -0x7fffffff;
        segs[0].last = -1;
        segs[1].first = view.width as i32;
        segs[1].last = 0x7fffffff;

        Self {
            segs,
            valid_segs_count: 2,
        }
    }
}
