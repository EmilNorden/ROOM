use crate::types::{DoomRealNum, real};
use crate::wad::{LumpStore, By, LumpNumber};
use std::io::Seek;
use std::mem::size_of;
use serde::Deserialize;

#[derive(Deserialize)]
struct SectorRaw {
    floor_height: i16,
    ceiling_height: i16,
    floor_pic: [u8; 8],
    ceiling_pic: [u8; 8],
    light_level: i16,
    special: i16,
    tag: i16,
}

pub struct Sector {
    floor_height: DoomRealNum,
    ceiling_height: DoomRealNum,
    floor_pic: usize,
    ceiling_pic: usize,
    light_level: i16,
    special: i16,
    tag: i16,

    // TODO: Unfinished, look at definition in original code
}

pub fn load(lumps: &LumpStore, map_lump: LumpNumber) -> Vec<Sector> {
    let mut data = lumps.get_lump_cursor(By::Number(map_lump.offset(8)));

    let sector_count = data.stream_len().unwrap() as usize / size_of::<SectorRaw>();
    let mut sectors = Vec::new();

    for _ in 0..sector_count {
        let raw_sector: SectorRaw = bincode::deserialize_from(&mut data).unwrap();
        sectors.push(Sector {
            floor_height: real(raw_sector.floor_height),
            ceiling_height: real(raw_sector.ceiling_height),

            floor_pic: 0,
            ceiling_pic: 0,
            light_level: 0,
            special: 0,
            tag: 0
        });
    }

    sectors
}