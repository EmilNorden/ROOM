use crate::types::{DoomRealNum, real};
use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::wad::{LumpStore, By, LumpNumber};

pub struct Blockmap<'a> {
    data: &'a [u8],
    origin_x: DoomRealNum,
    origin_y: DoomRealNum,
    width: DoomRealNum,
    height: DoomRealNum,
    // blocklinks: Vec<>, TODO SKIP FOR NOW
}

pub fn load(data: &[u8]) -> Blockmap {
    let mut cursor = Cursor::new(data);
    // TODO Blocklinks? See P_LoadBlockMap
    Blockmap {
        data,
        origin_x: real(cursor.read_i16::<LittleEndian>().unwrap()),
        origin_y: real(cursor.read_i16::<LittleEndian>().unwrap()),
        width: real(cursor.read_i16::<LittleEndian>().unwrap()),
        height: real(cursor.read_i16::<LittleEndian>().unwrap()),
    }
}