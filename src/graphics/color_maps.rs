use crate::wad::{By, LumpStore};

pub struct ColorMapData {
    color_maps: Vec<u8>,
}

impl ColorMapData {
    pub const NUM_COLOR_MAPS: usize = 32;

    pub fn init(lumps: &LumpStore) -> Self {
        let lump = lumps.get_lump(By::Name("COLORMAP"));
        let color_maps = Vec::from(lump);
        // TODO: The original code aligns color_maps to a 256 byte boundary. Needed?
        Self {
            color_maps
        }
    }

    pub fn color_maps(&self) -> &Vec<u8> { &self.color_maps }
}