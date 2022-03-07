use crate::graphics::color_maps::ColorMapData;
use crate::graphics::light_table::{DIST_MAP, LIGHT_LEVELS, MAX_LIGHT_SCALE};
use crate::graphics::textures::{TextureData};
use crate::number::RealNumber;
use crate::wad::LumpStore;
use crate::rendering::renderer::{RENDER_WIDTH, RENDER_HEIGHT};

pub mod bsp;
pub mod renderer;
pub(crate) mod patch;
mod palette;
pub mod types;
mod color_map;

pub struct ViewConfiguration {
    refresh_view_needed: bool,
    blocks: usize,
    detail: i32,
}

pub struct View {
    width: usize,
    height: usize,
    scaled_width: usize,
    centerx: usize,
    centery: usize,
    centerxfrac: RealNumber,
    centeryfrac: RealNumber,
    projection: RealNumber,
    detail_shift: i32,
    // scale_light: [[[u8; 256]; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize],
}

impl ViewConfiguration {
    pub fn new() -> Self {
        Self {
            refresh_view_needed: false,
            blocks: 9, // TODO: Implement something akin to M_LoadDefaults
            detail: 0,
        }
    }

    pub fn create_view(&mut self, /*color_map_data: &ColorMapData*/) -> View {
        self.refresh_view_needed = false;

        let (scaled_view_width, view_height) = if self.blocks == 11 {
            (RENDER_WIDTH, RENDER_HEIGHT)
        } else {
            (self.blocks * 32, (self.blocks * 168 / 10) & !7)
        };

        let detail_shift = self.detail;
        let view_width = scaled_view_width >> detail_shift;

        let centery = view_height / 2;
        let centerx = view_width / 2;
        let centerxfrac = RealNumber::new(centerx);
        let centeryfrac = RealNumber::new(centery);
        let projection = centerxfrac;

        // TODO This sets the drawing functions depending on detail level.
        // Lets skip this for now.
        /*if detail_shift == 0 {

        }
        else {

        }*/

        // Calculate the light levels to use
        // for each level / scale combination
        /*
        let mut scale_light = [[[0u8; 256]; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize];
        for i in 0..LIGHT_LEVELS as usize {
            let startmap = ((LIGHT_LEVELS - 1 - i) * 2) * ColorMapData::NUM_COLOR_MAPS / LIGHT_LEVELS;
            for j in 0..MAX_LIGHT_SCALE {
                let level = startmap - j * RENDER_WIDTH / (view_width << detail_shift) / DIST_MAP;

                let level = level.clamp(0, ColorMapData::NUM_COLOR_MAPS - 1);

                let color_map_offset = level as usize * 256;
                scale_light[i][j].clone_from_slice(&color_map_data.color_maps()[color_map_offset..color_map_offset+256]);
            }
        }*/


        View {
            width: view_width,
            height: view_height,
            scaled_width: scaled_view_width,
            centerx,
            centery,
            centerxfrac,
            centeryfrac,
            projection,
            detail_shift: self.detail,
            // scale_light,
        }
    }

    pub fn set_blocks(&mut self, blocks: usize) {
        self.blocks = blocks;
        self.refresh_view_needed = true;
    }

    pub fn set_detail(&mut self, detail: i32) {
        self.detail = detail;
        self.refresh_view_needed = true;
    }

    pub fn refresh_view_needed(&self) -> bool {
        self.refresh_view_needed
    }
}


impl View {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            scaled_width: 0,
            centerx: 0,
            centery: 0,
            centerxfrac: Default::default(),
            centeryfrac: Default::default(),
            projection: Default::default(),
            detail_shift: 0,
            // scale_light: [[[0u8; 256]; MAX_LIGHT_SCALE]; LIGHT_LEVELS as usize],
        }
    }

    // R_SetViewSize
    pub fn set_view_size(&mut self) {

    }

    // R_ExecuteSetViewSize
    pub fn execute_set_view_size(&mut self) {
    }
}

/*pub struct Patch {
    width: i16,
    height: i16,
    left_offset: i16,
    top_offset: i16,
    columnofs: [i32; 8],
}*/