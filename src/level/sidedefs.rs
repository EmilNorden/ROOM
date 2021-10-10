use crate::types::{DoomRealNum, real, WadString};
use crate::graphics::textures::{TextureNumber, TextureData};
use std::mem::size_of;
use std::io::Cursor;
use crate::rendering::renderer::Texture;
use serde::Deserialize;
use crate::wad::from_wad_string;
use crate::level::parse_entity_vector;

#[derive(Deserialize)]
struct RawSideDef {
    texture_offset: i16,
    row_offset: i16,
    top_texture: WadString,
    bottom_texture: WadString,
    mid_texture: WadString,
    sector: i16,
}

pub struct SideDef {
    // add this to the calculated texture column
    pub(crate) texture_offset: DoomRealNum,
    // add this to the calculated texture top
    pub(crate) row_offset: DoomRealNum,

    pub(crate) top_texture: TextureNumber,
    pub(crate) bottom_texture: TextureNumber,
    pub(crate) mid_texture: TextureNumber,

    // Sector the SideDef is facing.
    pub(crate) sector_index: usize,
}

pub fn load(data: &[u8], textures: &TextureData) -> Vec<SideDef> {
    parse_entity_vector(data, |x: RawSideDef| SideDef {
        texture_offset: real(x.texture_offset),
        row_offset: real(x.row_offset),
        top_texture: textures.get_texture_number(String::from(x.top_texture)).unwrap(),
        bottom_texture: textures.get_texture_number(String::from(x.bottom_texture)).unwrap(),
        mid_texture: textures.get_texture_number(String::from(x.mid_texture)).unwrap(),
        sector_index: x.sector as usize,
    })
}