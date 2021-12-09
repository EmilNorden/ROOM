use std::ops::Div;
use fixed::types::I16F16;
use image::error::UnsupportedErrorKind::Color;
use crate::graphics::color_maps::ColorMapData;
use crate::rendering::renderer::RENDER_WIDTH;
use crate::types::{real, real_from_bits, real_to_bits, real_to_int};

// I have no idea what these constants are
pub const LIGHT_LEVELS:i32 = 16;
pub const MAX_LIGHT_Z:usize = 128;
pub const LIGHT_Z_SHIFT:usize = 20;
pub const LIGHT_SCALE_SHIFT:usize = 12;
pub const DIST_MAP:i32 = 2;

pub struct LightTable {
    z_light: [[u8; MAX_LIGHT_Z]; LIGHT_LEVELS as usize],
}

impl LightTable {

    pub fn init(color_map_data: &ColorMapData) -> Self {
        let mut z_light = [[0u8; MAX_LIGHT_Z]; LIGHT_LEVELS as usize];
        for i in 0..LIGHT_LEVELS {
            let start_map = ((LIGHT_LEVELS - 1 - i) * 2) * ColorMapData::NUM_COLOR_MAPS / LIGHT_LEVELS;
            for j in 0..MAX_LIGHT_Z {
                let scale = real(RENDER_WIDTH / 2) / real_from_bits((j as i32 + 1) << 20);
                let scale = real_to_bits(scale >> LIGHT_SCALE_SHIFT);

                let mut level = if scale > start_map as i32 { 0 } else { start_map as i32 - scale / DIST_MAP };

                if level >= ColorMapData::NUM_COLOR_MAPS {
                    level = ColorMapData::NUM_COLOR_MAPS - 1;
                }

                z_light[i as usize][j] = color_map_data.color_maps()[level as usize * 256];
            }
        }

        Self {
            z_light
        }
    }
}