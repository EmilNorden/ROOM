use crate::graphics::color_maps::ColorMapData;
use crate::graphics::flats::FlatData;
use crate::graphics::sprites::SpriteData;
use crate::graphics::textures::TextureData;
use crate::wad::LumpStore;

pub mod textures;
pub mod flats;
pub mod sprites;
pub mod color_maps;
pub mod light_table;

pub struct GraphicsData {
    textures: TextureData,
    flats: FlatData,
    sprites: SpriteData,
    color_maps: ColorMapData,
}

impl GraphicsData {
    pub fn init(lumps: &LumpStore) -> Self {
        let textures = TextureData::init(lumps);
        let flats = FlatData::init(lumps);
        let sprites = SpriteData::init(lumps);
        let color_maps = ColorMapData::init(lumps);
        Self {
            textures,
            flats,
            sprites,
            color_maps
        }
    }

    pub fn textures(&self) -> &TextureData { &self.textures }

    pub fn flats(&self) -> &FlatData { &self.flats }

    pub fn color_maps(&self) -> &ColorMapData { &self.color_maps }
}