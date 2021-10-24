use crate::graphics::textures::{init_textures, TextureData};
use crate::types::DoomRealNum;
use crate::wad::LumpStore;

mod bsp;
pub mod renderer;
pub(crate) mod patch;
mod palette;
pub mod types;

pub struct View {
    width: usize,
    height: usize,
    scaled_width: usize,
    centerx: i32,
    centery: i32,
    centerxfrac: DoomRealNum,
    centeryfrac: DoomRealNum,
    projection: DoomRealNum,
}

pub struct RenderData {
    texture: TextureData
}

pub fn init_rendering(lumps: &LumpStore) -> RenderData {
    let texture = init_textures(lumps);

    RenderData {
        texture
    }
}

/*pub struct Patch {
    width: i16,
    height: i16,
    left_offset: i16,
    top_offset: i16,
    columnofs: [i32; 8],
}*/