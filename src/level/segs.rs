use serde::Deserialize;
use crate::types::{DoomRealNum, real};
use std::mem::size_of;
use std::io::Cursor;
use crate::level::parse_entity_vector;

#[derive(Deserialize)]
pub struct RawSeg {
    v1: i16,
    v2: i16,
    angle: i16,
    linedef: i16,
    side: i16,
    offset: i16,
}


pub struct Seg {
    pub(crate) vertex1_index: usize,
    pub(crate) vertex2_index: usize,
    pub(crate) offset: DoomRealNum,
    pub(crate) angle: u32,
    pub(crate) sidedef_index: usize,
    pub(crate) linedef_index: usize,

    pub(crate) front_sector_index: usize,
    pub(crate) back_sector_index: Option<usize>,
}

pub fn load(data: &[u8]) -> Vec<Seg> {
    parse_entity_vector(data, |raw_seg: RawSeg| Seg {
        vertex1_index: raw_seg.v1 as usize,
        vertex2_index: raw_seg.v2 as usize,
        offset: real((raw_seg.offset as u32) << 16),
        angle: (raw_seg.angle as u32) << 16,
        sidedef_index: raw_seg.side as usize,
        linedef_index: raw_seg.linedef as usize,
        front_sector_index: 1,
        back_sector_index: None
    })
}


// LineSeg, generated by splitting LineDefs
// using partition lines selected by BSP builder.