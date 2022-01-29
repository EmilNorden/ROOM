use crate::graphics::flats::FlatNumber;
use crate::level::Level;
use crate::level::sectors::Sector;
use crate::number::RealNumber;
use crate::rendering::renderer::{RENDER_HEIGHT, RENDER_WIDTH};
use crate::rendering::View;

pub struct Planes {
    pub floor_clip: [i16; RENDER_WIDTH],
    pub ceiling_clip: [i16; RENDER_WIDTH],
    pub visible_planes: [VisPlane; 128],
    pub last_visible_plane: usize,
    pub openings: [i16; RENDER_WIDTH * 64],
    pub last_opening: usize,
    pub cached_height: [RealNumber; RENDER_HEIGHT],
}

impl Planes {
    pub fn new(view: &View) -> Self {
        let floor = [view.height as i16; RENDER_WIDTH];
        let ceiling = [-1; RENDER_WIDTH];

        Self {
            floor_clip: floor,
            ceiling_clip: ceiling,
            visible_planes: [VisPlane::default(); 128],
            last_visible_plane: 0,
            openings: [0i16; RENDER_WIDTH * 64],
            last_opening: 0,
            cached_height: [RealNumber::new(0); RENDER_HEIGHT],
        }
    }
}

#[derive(Copy, Clone)]
pub struct VisPlane {
    pub height: RealNumber,
    pub picnum: FlatNumber,
    pub light_level: i16,
    pub min_x: i32,
    pub max_x: i32,

    pub pad1: u8,
    pub top: [u8; RENDER_WIDTH],
    pub pad2: u8,
    pub pad3: u8,
    pub bottom: [u8; RENDER_WIDTH],
    pub pad4: u8,
}

impl Default for VisPlane {
    fn default() -> Self {
        Self {
            height: Default::default(),
            picnum: FlatNumber(0),
            light_level: 0,
            min_x: 0,
            max_x: 0,
            pad1: 0,
            top: [0u8; RENDER_WIDTH],
            pad2: 0,
            pad3: 0,
            bottom: [0u8; RENDER_WIDTH],
            pad4: 0,
        }
    }
}