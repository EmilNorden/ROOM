use crate::wad::{LumpStore, By, LumpNumber};
use std::io::Seek;
use std::mem::size_of;
use serde::Deserialize;
use crate::graphics::flats::FlatNumber;
use crate::level::bounding_box::BoundingBox;
use crate::level::parse_entity_vector;
use crate::level::linedefs::LineDef;
use crate::number::RealNumber;
use crate::types::Vector3;

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
    pub(crate) floor_height: RealNumber,
    pub(crate) ceiling_height: RealNumber,
    pub(crate) floor_pic: FlatNumber,
    pub(crate) ceiling_pic: FlatNumber,
    pub(crate) light_level: i16,
    pub(crate) special: i16,
    pub(crate) tag: i16,
    pub(crate) line_count: u32,
    pub(crate) lines: Vec<LineDef>,
    pub(crate) soundorg: Vector3,
    pub(crate) blockbox: BoundingBox,

    // TODO: Unfinished, look at definition in original code
}

pub fn load(data: &[u8]) -> Vec<Sector> {
    parse_entity_vector(data, |raw_sector: RawSector| Sector {
        floor_height: RealNumber::new(raw_sector.floor_height),
        ceiling_height: RealNumber::new(raw_sector.ceiling_height),

        floor_pic: FlatNumber(0),
        ceiling_pic: FlatNumber(0),
        light_level: 0,
        special: 0,
        tag: 0,
        line_count: 0, // This will be updated later in the level loading functions
        lines: Vec::new(),
        soundorg: Vector3::default(),
        blockbox: BoundingBox::new_empty(),
    })
}