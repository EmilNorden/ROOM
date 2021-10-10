use crate::types::{DoomRealNum, real};
use crate::wad::{LumpStore, By, LumpNumber};
use std::io::Seek;
use std::mem::size_of;
use serde::Deserialize;
use crate::level::parse_entity_vector;
use crate::level::linedefs::LineDef;

#[derive(Deserialize)]
struct RawSector {
    floor_height: i16,
    ceiling_height: i16,
    floor_pic: [u8; 8],
    ceiling_pic: [u8; 8],
    light_level: i16,
    special: i16,
    tag: i16,
}

pub struct Sector {
    pub(crate) floor_height: DoomRealNum,
    pub(crate) ceiling_height: DoomRealNum,
    pub(crate) floor_pic: usize,
    pub(crate) ceiling_pic: usize,
    pub(crate) light_level: i16,
    pub(crate) special: i16,
    pub(crate) tag: i16,
    pub(crate) line_count: u32,
    pub(crate) lines: Vec<LineDef>,

    // TODO: Unfinished, look at definition in original code
}

pub fn load(data: &[u8]) -> Vec<Sector> {
    parse_entity_vector(data, |raw_sector: RawSector| Sector {
        floor_height: real(raw_sector.floor_height),
        ceiling_height: real(raw_sector.ceiling_height),

        floor_pic: 0,
        ceiling_pic: 0,
        light_level: 0,
        special: 0,
        tag: 0,
        line_count: 0, // This will be updated later in the level loading functions
        lines: Vec::new(),
    })
}