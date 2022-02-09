use std::ops::Div;
use fixed::types::I16F16;
use image::error::UnsupportedErrorKind::Color;
use crate::graphics::color_maps::ColorMapData;
use crate::number::RealNumber;
use crate::rendering::renderer::RENDER_WIDTH;

// I have no idea what these constants are
pub const LIGHT_SEG_SHIFT:usize = 4;
pub const MAX_LIGHT_SCALE:usize = 48;
pub const LIGHT_LEVELS:usize = 16;
pub const MAX_LIGHT_Z:usize = 128;
pub const LIGHT_Z_SHIFT:usize = 20;
pub const LIGHT_SCALE_SHIFT:i32 = 12;
pub const DIST_MAP:usize = 2;

pub struct LightTable {
    // scale_light: [[[u8; 256]; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize],
    z_light: [[[u8; 256]; MAX_LIGHT_Z]; LIGHT_LEVELS as usize],
}

impl LightTable {

    pub fn init(color_map_data: &ColorMapData) -> Self {
        const DIST_MAP:usize = 2;

        let mut z_light = [[[0u8; 256]; MAX_LIGHT_Z]; LIGHT_LEVELS as usize];
        let scale_light = [[0u8; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize];
        for i in 0..LIGHT_LEVELS {
            let start_map = ((LIGHT_LEVELS - 1 - i) * 2) * ColorMapData::NUM_COLOR_MAPS / LIGHT_LEVELS;
            for j in 0..MAX_LIGHT_Z {
                let scale = RealNumber::new(RENDER_WIDTH as i32 / 2) / RealNumber::new_from_bits((j as i32 + 1) << 20);
                let scale = scale >> LIGHT_SCALE_SHIFT;

                let mut level = start_map - scale.to_bits() as usize / DIST_MAP;
                level = level.clamp(0, ColorMapData::NUM_COLOR_MAPS - 1);
                let color_map_offset = level * 256;
                let dest = &mut z_light[i as usize][j];
                dest.clone_from_slice(&color_map_data.color_maps()[color_map_offset..color_map_offset+256]);
            }
        }

        Self {
            z_light,
            // scale_light: [[0u8; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize]
        }
    }
}