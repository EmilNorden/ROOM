use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::number::RealNumber;
use crate::wad::{LumpStore, By, LumpNumber};

pub struct Blockmap {
    data: Vec<u8>,
    origin_x: RealNumber,
    origin_y: RealNumber,
    width: RealNumber,
    height: RealNumber,
    // blocklinks: Vec<>, TODO SKIP FOR NOW
}

impl Blockmap {
    pub fn origin_x(&self) -> RealNumber { self.origin_x }
    pub fn origin_y(&self) -> RealNumber { self.origin_y }
    pub fn width(&self) -> RealNumber { self.width }
    pub fn height(&self) -> RealNumber { self.height }
}

pub fn load(data: &[u8]) -> Blockmap {
    let mut cursor = Cursor::new(data);
    let data = Vec::from(data);
    // TODO Blocklinks? See P_LoadBlockMap
    Blockmap {
        data,
        origin_x: RealNumber::new(cursor.read_i16::<LittleEndian>().unwrap()),
        origin_y: RealNumber::new(cursor.read_i16::<LittleEndian>().unwrap()),
        width: RealNumber::new(cursor.read_i16::<LittleEndian>().unwrap()),
        height: RealNumber::new(cursor.read_i16::<LittleEndian>().unwrap()),
    }
}