use crate::number::RealNumber;
use crate::rendering::patch::Patch;
use crate::wad::{By, LumpStore};

pub struct SpriteData {
    widths: Vec<RealNumber>,
    offsets: Vec<RealNumber>,
    top_offsets: Vec<RealNumber>,
}

impl SpriteData {
    pub fn init(lumps: &LumpStore) -> SpriteData {
        let first_sprite_lump = lumps.get_lump_number("S_START")
            .expect("Unable to find lump S_START").offset(1);

        let last_sprite_lump = lumps.get_lump_number("S_END")
            .expect("Unable to find lump S_END");

        let num_sprite_lumps = last_sprite_lump - first_sprite_lump;
        let mut sprite_width = Vec::with_capacity(num_sprite_lumps);
        let mut sprite_offset = Vec::with_capacity(num_sprite_lumps);
        let mut sprite_top_offset = Vec::with_capacity(num_sprite_lumps);

        for i in 0..num_sprite_lumps {
            let patch: Patch = lumps.get_lump(By::Number(first_sprite_lump.offset(i))).into();

            // TODO: Investigate if these should be converted to fixed point or not.
            sprite_width.push(RealNumber::new(patch.width()));
            sprite_offset.push(RealNumber::new(patch.left_offset()));
            sprite_top_offset.push(RealNumber::new(patch.top_offset()));
        }
        
        Self {
            widths: sprite_width,
            offsets: sprite_offset,
            top_offsets: sprite_top_offset
        }
    }
}