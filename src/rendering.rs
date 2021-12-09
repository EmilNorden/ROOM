use crate::graphics::textures::{TextureData};
use crate::types::{DoomRealNum, real};
use crate::wad::LumpStore;
use crate::rendering::renderer::{RENDER_WIDTH, RENDER_HEIGHT};

mod bsp;
pub mod renderer;
pub(crate) mod patch;
mod palette;
pub mod types;

pub struct ViewConfiguration {
    refresh_view_needed: bool,
    blocks: usize,
    detail: usize,
}

pub struct View {
    width: usize,
    height: usize,
    scaled_width: usize,
    centerx: i32,
    centery: i32,
    centerxfrac: DoomRealNum,
    centeryfrac: DoomRealNum,
    projection: DoomRealNum,
    detail_shift: usize,
}

impl ViewConfiguration {
    pub fn new() -> Self {
        Self {
            refresh_view_needed: false,
            blocks: 0,
            detail: 0,
        }
    }

    pub fn create_view(&mut self) -> View {
        self.refresh_view_needed = false;

        let (scaled_view_width, view_height) = if self.blocks == 11 {
            (RENDER_WIDTH, RENDER_HEIGHT)
        } else {
            (self.blocks * 32, (self.blocks * 168 / 10) & !7)
        };

        let detail_shift = self.detail;
        let view_width = scaled_view_width >> detail_shift;

        let center_y = view_height / 2;
        let center_x = view_width / 2;
        let center_x_frac = real(center_x);
        let center_y_frac = real(center_y);
        let projection = center_x_frac;

        // TODO This sets the drawing functions depending on detail level.
        // Lets skip this for now.
        /*if detail_shift == 0 {

        }
        else {

        }*/


        View {
            width: 0,
            height: 0,
            scaled_width: 0,
            centerx: 0,
            centery: 0,
            centerxfrac: Default::default(),
            centeryfrac: Default::default(),
            projection: Default::default(),
            detail_shift: 0,
        }
    }

    pub fn set_blocks(&mut self, blocks: usize) {
        self.blocks = blocks;
        self.refresh_view_needed = true;
    }

    pub fn set_detail(&mut self, detail: usize) {
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
            detail_shift: 0
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